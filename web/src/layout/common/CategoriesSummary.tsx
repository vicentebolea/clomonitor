import classNames from 'classnames';
import { RoundScore } from 'clo-ui/components/RoundScore';
import { isUndefined } from 'lodash';

import { CATEGORY_ICONS } from '../../data';
import { ScoreType } from '../../types';
import styles from './CategoriesSummary.module.css';
import CategoryProgressbar from './CategoryProgressbar';

interface Props {
  score: { [key in ScoreType]?: number };
  bigSize: boolean;
  repoName?: string;
  withLinks?: boolean;
  scrollIntoView?: (id?: string) => void;
}

const CategoriesSummary = (props: Props) => {
  const activeLink = !isUndefined(props.withLinks) && props.withLinks && props.repoName;

  return (
    <div
      className={classNames(
        'align-items-center d-flex flex-column flex-md-row',
        styles.summary,
        {
          'flex-lg-column flex-xl-row': !props.bigSize,
        },
        { [styles.bigSize]: props.bigSize }
      )}
    >
      <div
        className={classNames(
          'd-none d-md-block',
          { 'd-lg-none d-xl-block d-lg-none d-xl-block': !props.bigSize },
          { 'mx-3': props.bigSize }
        )}
      >
        <div className="d-flex flex-column me-0 me-sm-4 mb-2 mb-sm-0">
          <RoundScore score={props.score.global!} />
        </div>
      </div>

      <div
        className={classNames('flex-grow-1 w-100 position-relative', {
          'px-0 px-sm-3': props.bigSize,
        })}
      >
        <div className={classNames('row', { 'gx-4 gx-md-5': props.bigSize })}>
          <CategoryProgressbar
            name="Project"
            value={props.score.project}
            icon={CATEGORY_ICONS[ScoreType.Project]}
            bigSize={props.bigSize}
            linkTo={activeLink ? `${props.repoName}_${ScoreType.Project}` : undefined}
            scrollIntoView={activeLink ? props.scrollIntoView : undefined}
          />
          <CategoryProgressbar
            name="Source"
            value={props.score.source}
            icon={CATEGORY_ICONS[ScoreType.Source]}
            bigSize={props.bigSize}
            linkTo={activeLink ? `${props.repoName}_${ScoreType.Source}` : undefined}
            scrollIntoView={activeLink ? props.scrollIntoView : undefined}
          />
          <CategoryProgressbar
            name="Build"
            value={props.score.build}
            icon={CATEGORY_ICONS[ScoreType.Build]}
            bigSize={props.bigSize}
            linkTo={activeLink ? `${props.repoName}_${ScoreType.Build}` : undefined}
            scrollIntoView={activeLink ? props.scrollIntoView : undefined}
          />
        </div>
      </div>
    </div>
  );
};

export default CategoriesSummary;
