import './App.css'
import Login from './Login'
import { useAuth } from './AuthContext'

function App() {
  const { isAuthenticated } = useAuth();
  return (
    <>
      {isAuthenticated ? (
        <div style={{ color: 'white', padding: 16 }}>
          Logged in! คุณสามารถเรียก API อื่นๆ ต่อได้แล้ว
        </div>
      ) : (
        <Login/>
      )}
    </>
  )
}

export default App
