import './App.css';
import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';

import Header from "./Components/Header";
import Home from "./Pages/Home/Home";
import Algebra from "./Pages/Algebra/Algebra";

function App() {
  return (
    <Router>
      <Header />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/algebra" element={<Algebra />} />
      </Routes>
    </Router>
  );
}

export default App;
