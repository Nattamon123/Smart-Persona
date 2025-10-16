import './App.css'
import Login from './Login'
import { useAuth } from './AuthContext'
import Aichat from './AIchatbot'
import Register from './Register';
function App() {
  const { isAuthenticated } = useAuth();
  return (
    <>
      {isAuthenticated ? (
      <Aichat/>
      ) : (
        <Register/>
      )}
    </>
  )
}

export default App
