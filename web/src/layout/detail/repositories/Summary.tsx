import { CheckSetBadge } from 'clo-ui/components/CheckSetBadge';
import { isUndefined } from 'lodash';
import { VscGithub } from 'react-icons/vsc';
import { useLocation, useNavigate } from 'react-router-dom';

import { CATEGORY_ICONS } from '../../../data';
import { Repository, ScoreType } from '../../../types';
import getCheckSets from '../../../utils/getCheckSets';
import BadgeCell from './BadgeCell';
import styles from './Summary.module.css';

interface Props {
  repositories: Repository[];
  scrollIntoView: (id?: string) => void;
}

const Summary = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();

  const goToAnchor = (hash: string) => {
    props.scrollIntoView(`#${hash}`);
    navigate(
      {
        pathname: location.pathname,
        hash: hash,
      },
      { state: location.state }
    );
  };

  if (props.repositories.length === 0) return null;

  return (
    <div className="pt-2 mb-4 mb-md-5">
      <table data-testid="repositories-summary" className={`table table-bordered mb-0 w-100 ${styles.table}`}>
        <thead>
          <tr>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>
                <VscGithub />
              </small>
              <span>Repository</span>
            </th>
            <th scope="col" className="text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Global]}</small>
              <span className="d-inline-block d-md-none d-xl-inline-block ms-1 ms-xl-2">Global</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.CodeVulnerabilities]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Code Vuln.</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Maintenance]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Maintenance</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Testing]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Testing</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Source]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Source Risk</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Build]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Build Risk</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {props.repositories.map((repo: Repository) => {
            if (isUndefined(repo.report)) return null;
            const checkSets = getCheckSets(repo);
            return (
              <tr key={`summary_${repo.repository_id}`}>
                <td className={`align-middle ${styles.repoCell} ${styles.darkBgCell}`}>
                  <div className="d-flex flex-row align-items-center px-2">
                    <button
                      className={`btn btn-link text-dark text-start text-truncate fw-bold ps-0 pe-2 py-0 py-xl-1 ${styles.repoBtn}`}
                      onClick={() => goToAnchor(repo.name)}
                      aria-label={`Go from summary to section: ${repo.name}`}
                    >
                      {repo.name}
                    </button>
                    <CheckSetBadge checkSets={checkSets} className="d-none d-xl-inline-flex" />
                  </div>
                </td>

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.global : undefined}
                  cellClassName="align-middle"
                  onClick={() => goToAnchor(repo.name)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.code_vulnerabilities : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.CodeVulnerabilities}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.maintenance : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.Maintenance}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.testing : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.Testing}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.source : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.Source}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.build : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.Build}`)}
                />
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

export default Summary;
