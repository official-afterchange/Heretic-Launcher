import { t } from '../lib/i18n';
import './account.css';

export default function Account() {
  const handleSaveUsername = () => {
    // Action lors de la mise à jour du pseudo (ex: stockage ou appel Rust)
    console.log("Mise à jour du pseudo demandée...");
  };

  return (
    <div id="account">
      <header className="account-header">
        <h1>{t('ui.account.title')}</h1>
        <p>{t('ui.account.description')}</p>
      </header>

      <div className="account-content">
        <div className="skin-preview">
          <div className="face-container">
            <img src="./assets/img/face.jpg" alt="Player Face" />
          </div>
          <p>{t('ui.account.skin.overview')}</p>
        </div>

        <div className="account-form">
          <div className="form-group">
            <label htmlFor="username-input">{t('ui.account.user.username')}</label>
            <input type="text" id="username-input" placeholder="Entrez votre pseudo..." />
          </div>
          <button id="save-username" className="action-button" onClick={handleSaveUsername}>
            {t('ui.account.user.update')}
          </button>
        </div>
      </div>
    </div>
  );
}