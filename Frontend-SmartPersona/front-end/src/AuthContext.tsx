import { createContext, useCallback, useContext, useMemo, useState } from 'react';

type AuthContextValue = {
  isAuthenticated: boolean;
  setAuthenticated: (value: boolean) => void;
};

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
    const [isAuthenticated, setIsAuthenticated] = useState(() => {
        return localStorage.getItem("auth") === "true";
      });
      
      const setAuthenticated = useCallback((value: boolean) => {
        setIsAuthenticated(value);
        localStorage.setItem("auth", String(value));
      }, []);

  const value = useMemo(() => ({ isAuthenticated, setAuthenticated }), [isAuthenticated, setAuthenticated]);

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within an AuthProvider');
  return ctx;
}


