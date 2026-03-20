import { CATEGORY_ICONS } from '../../data';
import { ScoreType } from '../../types';
import styles from './Average.module.css';
import ProgressBar from './ProgressBar';

interface Props {
  title: string;
  data: { [key in ScoreType]: number };
}

const Average = (props: Props) => {
  return (
    <>
      <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>{props.title}</div>
      <div className="card-body pt-2 pt-md-3 px-3 px-md-4 pb-0">
        <ProgressBar
          title="Code Vulnerabilities"
          icon={CATEGORY_ICONS[ScoreType.CodeVulnerabilities]}
          value={props.data.code_vulnerabilities}
        />
        <ProgressBar
          title="Maintenance"
          icon={CATEGORY_ICONS[ScoreType.Maintenance]}
          value={props.data.maintenance}
        />
        <ProgressBar
          title="Continuous Testing"
          icon={CATEGORY_ICONS[ScoreType.Testing]}
          value={props.data.testing}
        />
        <ProgressBar
          title="Source Risk"
          icon={CATEGORY_ICONS[ScoreType.Source]}
          value={props.data.source}
        />
        <ProgressBar
          title="Build Risk"
          icon={CATEGORY_ICONS[ScoreType.Build]}
          value={props.data.build}
        />
      </div>
    </>
  );
};

export default Average;
