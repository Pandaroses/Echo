import './App.css'
import {
  BrowserRouter,
  Routes,
  Route,
} from "react-router-dom";
import { Home, Login, Register } from "./pages/index"

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="*" element={<div> Not Found </div>} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
