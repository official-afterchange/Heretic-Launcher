import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { t } from '../lib/i18n';
import { useServerStatus } from '../hooks/useServerStatus';
import './Home.css';
import { useApp } from '../context/AppContext';

type LaunchProgress = {
  step: string;
  message: string;
};

export default function Home() {
  const { instance, config } = useApp();
  const { status, loading } = useServerStatus(instance?.server_address);
  const [preparing, setPreparing] = useState(false);
  const [gameRunning, setGameRunning] = useState(false);
  const [preparationMessage, setPreparationMessage] = useState('');

  useEffect(() => {
    let cancelled = false;
    let unlisten: (() => void) | undefined;

    listen<LaunchProgress>('launch-progress', ({ payload }) => {
      setPreparationMessage(payload.message);
      if (payload.step === 'started') setGameRunning(true);
      if (payload.step === 'stopped' || payload.step === 'error') setGameRunning(false);
    }).then((stopListening) => {
      if (cancelled) stopListening();
      else unlisten = stopListening;
    }).catch((error: unknown) => {
      console.error('Impossible d’écouter la progression du téléchargement :', error);
    });

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  const handlePlayClick = async () => {
    if (preparing || gameRunning) return;

    setPreparing(true);
    setPreparationMessage(t('ui.home.download.starting'));
    try {
      const username = config?.player_username.trim() ?? '';
      if (!username) throw new Error(t('ui.home.download.usernameRequired'));
      await invoke('launch_instance');
      setPreparationMessage(t('ui.home.download.started'));
      setGameRunning(true);
    } catch (error) {
      console.error('Échec de la préparation de Minecraft :', error);
      setPreparationMessage(`${t('ui.home.download.error')} ${String(error)}`);
    } finally {
      setPreparing(false);
    }
  };

  return (
    <div id="home">
      <div className="home-grid">
        <div className="hero-section">
          <div id="statut"><span className="status-dot"></span>
          {loading ? (
            <p>searching</p>
          ) : status?.online ? (
            <p>online</p>
          ) : (
            <p>offline</p>
          )}
          </div>

          <div className="main-content">
            <div className="logo-container">
              <img src="./assets/img/logo.png" alt="Butterfly Logo" />
            </div>

            <button className="play-button" onClick={handlePlayClick} disabled={preparing || gameRunning}>
              <img src="./assets/img/play.png" alt="" />
              <b>
                <p>{preparing ? t('ui.home.download.preparing') : gameRunning ? t('ui.home.download.running') : t('ui.home.play')}</p>
              </b>
            </button>
            {preparationMessage && (
              <div className={`download-status${preparing ? ' is-active' : ''}`} role="status" aria-live="polite">
                {preparing && <span className="download-spinner" aria-hidden="true" />}
                <span>{preparationMessage}</span>
              </div>
            )}
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
            
            {loading ? (
              <div className="server-stats">
                <p className="stat-label">Chargement...</p>
              </div>
            ) : status?.online ? (
              <div className="server-stats">
                <div className="stat">
                  <span className="stat-label">{t('ui.home.players')} {t('status.online')}</span>
                  <span className="stat-value" id="count-players">{status.players_online} / {status.players_max}</span>
                </div>
                <div className="stat">
                  <span className="stat-label">{t('ui.home.ping')} {t('status.average')}</span>
                  <span className="stat-value" id="ping-value">{status.ping_ms}ms</span>
                </div>
              </div>
            ) : (
              <div className="server-stats">
                <p className="stat-label">Hors Ligne</p>
              </div>
            )}
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