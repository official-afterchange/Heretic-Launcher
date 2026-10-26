import { useState, useEffect } from 'react';
import './Settings.css';

import { t, setLanguage, getLanguage } from '../lib/i18n'; 

export default function Settings() {
    const [selectedLang, setSelectedLang] = useState(getLanguage());

    // Écoute les changements globaux de langue pour garder le select synchronisé
    useEffect(() => {
        const handleLangChange = () => {
        setSelectedLang(getLanguage());
        };
        
        window.addEventListener('languagechange', handleLangChange);
        return () => window.removeEventListener('languagechange', handleLangChange);
    }, []);

    // Fonction appelée directement quand l'utilisateur choisit une option
    const handleLanguageSelect = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const newLang = e.target.value;
        setSelectedLang(newLang);
        setLanguage(newLang); // <-- Applique le changement immédiatement dans i18n
    };

  return (
    <div id="settings">
      <header className="settings-header">
        <h1>{t('ui.settings.general.title')}</h1>
        <p>{t('ui.settings.general.description')}</p>
      </header>

      <div className="settings-content">
        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="server-url-input">{t('ui.settings.sync.title')}</label>
            </div>
            <input 
              type="text" 
              id="server-url-input" 
              className="settings-input"
              placeholder="http://185.146... (Laisser vide pour désactiver)" 
            />
            <p className="settings-description">{t('ui.settings.sync.description')}</p>
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="mc-ip-input">{t('ui.settings.mc_ip.title')}</label>
            </div>
            <input 
              type="text" 
              id="mc-ip-input" 
              className="settings-input"
              placeholder="play.server.com (Laisser vide pour désactiver)" 
            />
            <p className="settings-description">{t('ui.settings.mc_ip.description')}</p>
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="game-dir-input">{t('ui.settings.folder.title')}</label>
            </div>
            <input type="text" id="game-dir-input" className="settings-input" placeholder=".butterfly" />
            <p className="settings-description">{t('ui.settings.folder.description')}</p>
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="lang-select">{t('ui.settings.lang.title')}</label>
            </div>
            <select 
              id="lang-select" 
              className="settings-select" 
              value={selectedLang} 
              onChange={handleLanguageSelect}
            >
              <option value="en">English</option>
              <option value="fr">Français</option>
              <option value="es">Español</option>
              <option value="de">Deutsch</option>
              <option value="ar">العربية</option>
            </select>
            <p className="settings-description">{t('ui.settings.lang.description')}</p>
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="game-version-input">{t('ui.settings.minecraft.title')}</label>
            </div>
            <input type="text" id="game-version-input" className="settings-input" placeholder="1.20.1" />
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="loader-select">{t('ui.settings.loader.title')}</label>
            </div>
            <select id="loader-select" className="settings-select">
              <option value="vanilla">Vanilla</option>
              <option value="fabric">Fabric</option>
            </select>
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="exec-path-input">{t('ui.settings.java.title')}</label>
            </div>
            <input type="text" id="exec-path-input" className="settings-input" placeholder="java" />
            <p className="settings-description">{t('ui.settings.java.description')}</p>
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="ram-slider">{t('ui.settings.ram.title')}</label>
              <span id="ram-value">4 GB</span>
            </div>
            <input type="range" id="ram-slider" min="2" max="16" step="1" defaultValue="4" />
            <div className="range-labels">
              <span>2 GB</span>
              <span id="ram-max-label">16 GB</span>
            </div>
            <p className="settings-description">
              {t('ui.settings.ram.description')}
            </p>
          </div>
        </div>
        <div className="separator"></div>

        <div className="settings-group">
          <div className="settings-item">
            <div className="item-info">
              <label htmlFor="mod-file-input">{t('ui.settings.mods.title')}</label>
            </div>
            <p className="settings-description">
              {t('ui.settings.mods.description')}
            </p>
            <input type="file" id="mod-file-input" accept=".jar" style={{ display: 'none' }} multiple />
            <div id="mods-dropzone" className="dropzone" title="Cliquez pour choisir un mod">
              <p>{t('ui.settings.mods.add')}</p>
            </div>
            <ul id="local-mods-list" className="mods-list"></ul>
          </div>
        </div>
      </div>
    </div>
  );
}