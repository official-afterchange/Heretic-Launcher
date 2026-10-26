import { t } from '../lib/i18n';
import './info.css';

export default function Info() {
  return (
    <div id="info">
      <div className="info-container">
        <div className="profile-card">
          <div className="avatar-wrapper">
            <img src="./assets/img/afterchange.jpg" alt="Developer Logo" />
          </div>
          <h1>{t('ui.info.header.name')}</h1>
          <p className="role">{t('ui.info.header.role')}</p>

          <div className="social-links">
            <a href="#" title="Discord"><img src="./assets/img/discord.png" alt="Discord" /></a>
            <a href="#" title="YouTube"><img src="./assets/img/youtube.png" alt="YouTube" /></a>
            <a href="#" title="X"><img src="./assets/img/x.png" alt="X" /></a>
            <a href="#" title="GitHub"><img src="./assets/img/internet.png" alt="GitHub" /></a>
          </div>
        </div>

        <div className="about-section">
          <header>
            <h2>{t('ui.info.content.overview')}</h2>
          </header>
          <p>
            {t('ui.info.content.description')}
          </p>
          <p>
            {t('ui.info.content.stack')}
          </p>

          <div className="stats-grid">
            <div className="stat-item">
              <span className="stat-value">{t('ui.info.version.value')}</span>
              <span className="stat-label">{t('ui.info.version.title')}</span>
            </div>
            <div className="stat-item">
              <span className="stat-value">{t('ui.info.version.build_value')}</span>
              <span className="stat-label">{t('ui.info.version.build_label')}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}