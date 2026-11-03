import { useEffect, useRef, useState } from 'react';
import { useApp } from '../context/AppContext';
import { t } from '../lib/i18n';
import SkinViewer from '../components/skinviewer';
import './account.css';

type AccountProfile = {
  id: string;
  username: string;
};

function loadAccounts(): AccountProfile[] {
  if (typeof window === 'undefined') return [{ id: 'local-account', username: '' }];

  try {
    const stored: unknown = JSON.parse(localStorage.getItem('heretic_accounts') ?? 'null');
    if (Array.isArray(stored)) {
      const profiles = stored.filter((profile): profile is AccountProfile =>
        typeof profile === 'object' && profile !== null &&
        'id' in profile && typeof profile.id === 'string' &&
        'username' in profile && typeof profile.username === 'string',
      );
      if (profiles.length > 0) return profiles;
    }
  } catch {
    // Ignore invalid local data and start with an empty local profile.
  }

  return [{ id: 'local-account', username: '' }];
}

export default function Account() {
  const { config, updateConfig } = useApp();
  const [accounts, setAccounts] = useState<AccountProfile[]>(loadAccounts);
  const [activeAccountId, setActiveAccountId] = useState(() => {
    const storedId = typeof window === 'undefined' ? null : localStorage.getItem('heretic_active_account');
    return accounts.find((account) => account.id === storedId)?.id ?? accounts[0].id;
  });
  const [saved, setSaved] = useState(false);
  const usernameInitialized = useRef(false);
  const activeAccount = accounts.find((account) => account.id === activeAccountId) ?? accounts[0];

  useEffect(() => {
    if (!config || usernameInitialized.current) return;
    usernameInitialized.current = true;

    const savedUsername = config.player_username.trim();
    if (savedUsername) {
      setAccounts((current) => current.map((account) =>
        account.id === activeAccountId ? { ...account, username: savedUsername } : account,
      ));
    } else if (activeAccount.username.trim()) {
      // Migre le pseudo déjà enregistré localement vers le fichier de configuration AppData.
      void updateConfig({ player_username: activeAccount.username.trim() }).catch((error: unknown) => {
        console.error('Impossible de migrer le pseudo du joueur :', error);
      });
    }
  }, [activeAccount.username, activeAccountId, config, updateConfig]);

  const updateUsername = (username: string) => {
    setSaved(false);
    setAccounts((current) => current.map((account) =>
      account.id === activeAccount.id ? { ...account, username } : account,
    ));
  };

  const addAccount = () => {
    const account = { id: `local-account-${Date.now()}`, username: '' };
    setAccounts((current) => [...current, account]);
    setActiveAccountId(account.id);
    setSaved(false);
  };

  const saveAccounts = async () => {
    const username = activeAccount.username.trim();
    try {
      await updateConfig({ player_username: username });
    } catch (error) {
      console.error('Impossible d’enregistrer le pseudo du joueur :', error);
      return;
    }
    localStorage.setItem('heretic_accounts', JSON.stringify(accounts));
    localStorage.setItem('heretic_active_account', activeAccountId);
    setSaved(true);
  };

  return (
    <div id="account">
      <header className="account-header">
        <div>
          <span className="account-eyebrow">{t('ui.account.eyebrow')}</span>
          <h1>{t('ui.account.title')}</h1>
          <p>{t('ui.account.description')}</p>
        </div>
      </header>

      <div className="account-layout">
        <aside className="account-list-panel">
          <div className="account-list-heading">
            <div>
              <h2>{t('ui.account.list.title')}</h2>
              <p>{t('ui.account.list.count', { count: accounts.length })}</p>
            </div>
            <span className="account-count">{accounts.length}</span>
          </div>

          <div className="account-list">
            {accounts.map((account) => {
              const isActive = account.id === activeAccountId;
              const displayName = account.username.trim() || t('ui.account.list.unnamed');

              return (
                <button
                  className={`account-item${isActive ? ' is-active' : ''}`}
                  key={account.id}
                  onClick={() => {
                    setActiveAccountId(account.id);
                    setSaved(false);
                  }}
                  type="button"
                  aria-pressed={isActive}
                >
                  <span className="account-avatar">{account.username.trim().charAt(0).toUpperCase() || '✦'}</span>
                  <span className="account-item-copy">
                    <span className="account-item-name">{displayName}</span>
                    <span className="account-item-type">{t('ui.account.list.local')}</span>
                  </span>
                  {isActive && <span className="account-active-dot" aria-label={t('ui.account.list.active')} />}
                </button>
              );
            })}
          </div>

          <button className="add-account-button" onClick={addAccount} type="button">
            <span aria-hidden="true">+</span>
            {t('ui.account.list.add')}
          </button>
          <p className="account-list-note">{t('ui.account.list.note')}</p>
        </aside>

        <section className="account-details" aria-labelledby="account-details-title">
          <div className="account-details-heading">
            <div>
              <span className="account-eyebrow">{t('ui.account.details.eyebrow')}</span>
              <h2 id="account-details-title">{activeAccount.username.trim() || t('ui.account.list.unnamed')}</h2>
            </div>
            <span className="active-account-badge"><span />{t('ui.account.list.active')}</span>
          </div>

          <div className="account-profile-content">
            <div className="skin-preview">
              <SkinViewer />
              <p>{t('ui.account.skin.overview')}</p>
            </div>

            <div className="account-form">
              <div className="form-intro">
                <h3>{t('ui.account.user.title')}</h3>
                <p>{t('ui.account.user.description')}</p>
              </div>
              <div className="form-group">
                <label htmlFor="username-input">{t('ui.account.user.username')}</label>
                <input
                  type="text"
                  id="username-input"
                  placeholder={t('ui.account.user.placeholder')}
                  value={activeAccount.username}
                  maxLength={16}
                  onChange={(event) => updateUsername(event.target.value)}
                />
                <span className="input-hint">{t('ui.account.user.hint')}</span>
              </div>
              <div className="account-save-row">
                <span className="local-account-label">{t('ui.account.list.local')}</span>
                <button id="save-username" className="action-button" onClick={saveAccounts} type="button">
                  {t(saved ? 'ui.account.user.saved' : 'ui.account.user.update')}
                  <span aria-hidden="true">→</span>
                </button>
              </div>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}