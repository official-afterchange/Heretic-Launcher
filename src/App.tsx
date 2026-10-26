import "./App.css";
import { BrowserRouter, Routes, Route, Link } from 'react-router-dom';

// Importation des vues séparées
import Layout from './layout';
import Home from './pages/home';
import Info from './pages/info';
import Browser from './pages/browser';
import Account from './pages/account';
import Settings from './pages/settings';


function App() {
  return (
    <BrowserRouter>
      {/* Routes */}
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<Home />} />
          <Route path="/info" element={<Info />} />
          <Route path="/browser" element={<Browser />} />
          <Route path="/account" element={<Account />} />
          <Route path="/settings" element={<Settings />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;