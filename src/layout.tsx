import { useState, useEffect } from 'react';
import { Outlet, Link, useLocation } from 'react-router-dom';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { t } from './lib/i18n';
import '../public/assets/styles.css'; // Ou ton fichier CSS global

export default function layout() {
  const location = useLocation();
  const [appWindow, setAppWindow] = useState<ReturnType<typeof getCurrentWindow> | null>(null);
  const [isMaximized, setIsMaximized] = useState<boolean>(false);

  useEffect(() => {
    // Vérifie si on est dans l'environnement Tauri
    if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
      return;
    }

    const win = getCurrentWindow();
    setAppWindow(win);

    win.isMaximized().then((maximized) => {
      setIsMaximized(maximized);
    });

    let unlistenFn: (() => void) | undefined;

    win.onResized(async () => {
      const maximized = await win.isMaximized();
      setIsMaximized(maximized);
    }).then((fn) => {
      unlistenFn = fn;
    });

    return () => {
      if (unlistenFn) unlistenFn();
    };
  }, []);

  const minimizeWindow = () => {
    if (!appWindow) return;
    void appWindow.minimize();
  };

  const toggleFullScreen = () => {
    if (!appWindow) return;
    void appWindow.toggleMaximize();
  };

  const fermerFenetre = () => {
    if (!appWindow) return;
    void appWindow.close();
  };

  return (
    <div id="window" className={isMaximized ? 'maximized' : ''}>
      <nav id="nav">
        <div className="nav-top">
          <Link 
            to="/" 
            className={`button ${location.pathname === '/' ? 'active' : ''}`}
          >
            <img src="/assets/img/maison.png" alt="Home" />
          </Link>

          <Link 
            to="/info" 
            className={`button ${location.pathname === '/info' ? 'active' : ''}`}
          >
            <img src="/assets/img/info.png" alt="Info" />
          </Link>

          <Link 
            to="/browser" 
            className={`button ${location.pathname === '/browser' ? 'active' : ''}`}
          >
            <img src="/assets/img/internet.png" alt="Browser" />
          </Link>
        </div>

        <div className="nav-bottom">
          <Link 
            to="/account" 
            className={`button ${location.pathname === '/account' ? 'active' : ''}`}
          >
            <img src="/assets/img/compte.png" alt="Account" />
          </Link>

          <Link 
            to="/settings" 
            className={`button ${location.pathname === '/settings' ? 'active' : ''}`}
            id="parametre"
          >
            <img src="/assets/img/parametre.png" alt="Settings" />
          </Link>
        </div>
      </nav>

      <main id="page">
        <header id="bar">
          <div id="player">
            <img src="/assets/img/face.jpg" alt="Player Face" />
            <h1 id="layout-username">Player</h1>
          </div>

          <div id="reseaux">
            <a href="https://youtube.com" target="_blank" rel="noreferrer" title="YouTube">
              <img src="/assets/img/youtube.png" alt="YouTube" />
            </a>
            <a href="https://discord.gg" target="_blank" rel="noreferrer" title="Discord">
              <img src="/assets/img/discord.png" alt="Discord" />
            </a>
            <a href="https://google.com" target="_blank" rel="noreferrer" title="Website">
              <img src="/assets/img/internet.png" alt="Web" />
            </a>
            <a href="https://x.com" target="_blank" rel="noreferrer" title="X">
              <img src="/assets/img/x.png" alt="X" />
            </a>
            <a href="https://tiktok.com" target="_blank" rel="noreferrer" title="TikTok">
              <img src="/assets/img/tiktok.png" alt="TikTok" />
            </a>
          </div>

          <div id="gestion">
            <button type="button" className="btn-action" onClick={minimizeWindow}>
              <img id="minimize-img" src="/assets/img/trait.png" alt="Minimize" />
            </button>
            <button type="button" className="btn-action" onClick={toggleFullScreen}>
              <img id="aggrandir-img" src="/assets/img/carre.png" alt="Maximize" />
            </button>
            <button type="button" className="btn-action" onClick={fermerFenetre}>
              <img id="close-img" src="/assets/img/croix.png" alt="Close" />
            </button>
          </div>
        </header>

        <section id="view-port">
          {/* C'est ici que React Router affichera la vue active (Home, Settings, Info, etc.) */}
          <Outlet />
        </section>

        <footer id="footer">
          <p>{t('ui.info.version.value')}</p>
          <p>{t('footer.not_affiliated')}</p>
          <p>{t('footer.copyright')}</p>
        </footer>
      </main>
    </div>
  );
}