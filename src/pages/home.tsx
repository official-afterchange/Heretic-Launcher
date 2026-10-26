import { t } from '../lib/i18n';
import './Home.css';

export default function Home() {
  const handlePlayClick = () => {
    console.log("Lancement du jeu demandé...");
  };

  return (
    <div id="home">
      <div className="home-grid">
        <div className="hero-section">
          <div id="statut"><span className="status-dot"></span>searching</div>

          <div className="main-content">
            <div className="logo-container">
              <img src="./assets/img/logo.png" alt="Butterfly Logo" />
            </div>

            <button className="play-button" onClick={handlePlayClick}>
              <img src="./assets/img/play.png" alt="" />
              <b>
                <p>{t('ui.home.play')}</p>
              </b>
            </button>
          </div>

          <p id="message">{t('dynamic.version_label')}</p>
        </div>

        <div className="sidebar-section">
          <div className="info-card news-card">
            <h3>{t('ui.home.news')}</h3>
            <div className="news-items">
              {/* Contenu dynamique des news */}
            </div>
          </div>

          <div className="info-card server-info-card">
            <h3>{t('ui.home.server')}</h3>
            <div className="server-stats">
              <div className="stat">
                <span className="stat-label">{t('ui.home.players')} {t('status.online')}</span>
                <span className="stat-value" id="count-players">0 / 0</span>
              </div>
              <div className="stat">
                <span className="stat-label">{t('ui.home.ping')} {t('status.average')}</span>
                <span className="stat-value" id="ping-value">0ms</span>
              </div>
            </div>
            <div className="quick-actions">
              <a href="#" className="mini-btn"><img src="./assets/img/discord.png" alt="" /> Discord</a>
              <a href="#" className="mini-btn"><img src="./assets/img/internet.png" alt="" /> Site Web</a>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}