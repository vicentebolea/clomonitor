-- Returns some stats in json format.
create or replace function get_stats(p_foundation text)
returns json as $$
    set local timezone to 'utc';

    with ratings as (
        select p.maturity, p.rating, count(*) as total
        from project p
        where p.rating is not null
        and p.foundation_id = p_foundation
        group by p.maturity, p.rating
    )
    select json_strip_nulls(json_build_object(
        'generated_at', floor(extract(epoch from current_timestamp) * 1000),
        'snapshots', (
            select json_agg(s.date)
            from (
                select date
                from stats_snapshot
                where foundation_id = p_foundation
                order by date desc
            ) s
        ),
        'projects', json_build_object(
            'running_total', (
                select json_agg(json_build_array(
                    floor(extract(epoch from projects_month) * 1000),
                    running_total
                ))
                from (
                    select
                        projects_month,
                        sum(total) over (order by projects_month asc) as running_total
                    from (
                        select
                            date_trunc('month', p.accepted_at) as projects_month,
                            count(*) as total
                        from project p
                        where p.accepted_at is not null
                        and p.foundation_id = p_foundation
                        group by date_trunc('month', p.accepted_at)
                    ) mt
                ) rt
            ),
            'accepted_distribution', (
                select json_agg(row_to_json(entry_count))
                from (
                    select
                        extract('year' from p.accepted_at) as year,
                        extract('month' from p.accepted_at) as month,
                        count(*) as total
                    from project p
                    where p.accepted_at is not null
                    and p.foundation_id = p_foundation
                    group by
                        extract('year' from p.accepted_at),
                        extract('month' from p.accepted_at)
                    order by year desc, month desc
                ) entry_count
            ),
            'rating_distribution', (
                select json_object_agg(maturity, rating_totals)
                from (
                    (
                        select
                            'all' as maturity,
                            jsonb_agg(jsonb_build_object(rating, total)) as rating_totals
                        from (
                            select rating, sum(total) as total
                            from ratings
                            group by rating
                            order by rating asc
                        ) as all_rating_totals
                    )
                    union
                    (
                        select
                            maturity,
                            jsonb_agg(jsonb_build_object(rating, total)) as rating_totals
                        from (
                            select maturity, rating, sum(total) as total
                            from ratings
                            where maturity is not null
                            group by maturity, rating
                            order by maturity, rating asc
                        ) as maturity_rating_totals
                        group by maturity
                        order by maturity asc
                    )
                ) as rating_distribution
            ),
            'sections_average', (
                select json_object_agg(maturity, sections_average)
                from (
                    (
                        select
                            'all' as maturity,
                            (
                                select jsonb_build_object(
                                    'project', (average_section_score(p_foundation, 'project', null)),
                                    'source', (average_section_score(p_foundation, 'source', null)),
                                    'build', (average_section_score(p_foundation, 'build', null))
                                ) as sections_average
                            )
                        from project
                    )
                    union
                    (
                        select
                            distinct maturity,
                            (
                                select jsonb_build_object(
                                    'project', (average_section_score(p_foundation, 'project', maturity)),
                                    'source', (average_section_score(p_foundation, 'source', maturity)),
                                    'build', (average_section_score(p_foundation, 'build', maturity))
                                ) as sections_average
                            )
                        from project
                        where maturity is not null
                    )
                ) sections_average
            ),
            'views_daily', (
                select json_agg(json_build_array(extract(epoch from day)*1000, total))
                from (
                    select pv.day, sum(pv.total) as total
                    from project_views pv
                    join project p using (project_id)
                    where pv.day >= current_date - '1 month'::interval
                    and p.foundation_id = p_foundation
                    group by day
                    order by day asc
                ) dt
            ),
            'views_monthly', (
                select json_agg(json_build_array(extract(epoch from month)*1000, total))
                from (
                    select date_trunc('month', pv.day) as month, sum(pv.total) as total
                    from project_views pv
                    join project p using (project_id)
                    where pv.day >= current_date - '2 year'::interval
                    and p.foundation_id = p_foundation
                    group by month
                    order by month asc
                ) mt
            )
        ),
        'repositories', json_build_object(
            'passing_check', json_build_object(
                'project', json_build_object(
                    'maintained', repositories_passing_check(p_foundation, 'project', 'maintained')
                ),
                'source', json_build_object(
                    'code_review', repositories_passing_check(p_foundation, 'source', 'code_review'),
                    'dangerous_workflow', repositories_passing_check(p_foundation, 'source', 'dangerous_workflow'),
                    'token_permissions', repositories_passing_check(p_foundation, 'source', 'token_permissions')
                ),
                'build', json_build_object(
                    'binary_artifacts', repositories_passing_check(p_foundation, 'build', 'binary_artifacts'),
                    'dependency_update_tool', repositories_passing_check(p_foundation, 'build', 'dependency_update_tool'),
                    'signed_releases', repositories_passing_check(p_foundation, 'build', 'signed_releases')
                )
            )
        )
    ));
$$ language sql;
