import styled from 'styled-components';
import { useEffect, useState } from 'react';

interface User {
  id: string;
  username: string;
  first_name: string;
  last_name: string;
  created_at: string;
  updated_at: string;
}

const UsersList = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const API_BASE = (import.meta as any).env?.VITE_BACKEND_URL || 'http://localhost:3000';

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoading(true);
        const response = await fetch(`${API_BASE}/users`, {
          method: 'GET',
          credentials: 'include',
        });

        if (!response.ok) {
          throw new Error('Failed to fetch users');
        }

        const data = await response.json();
        setUsers(data);
      } catch (err: any) {
        setError(err.message || 'Failed to load users');
      } finally {
        setLoading(false);
      }
    };

    fetchUsers();
  }, [API_BASE]);

  if (loading) {
    return (
      <StyledWrapper>
        <div className="loading">กำลังโหลดข้อมูล...</div>
      </StyledWrapper>
    );
  }

  if (error) {
    return (
      <StyledWrapper>
        <div className="error">เกิดข้อผิดพลาด: {error}</div>
      </StyledWrapper>
    );
  }

  return (
    <StyledWrapper>
      <div className="container">
        <h1>รายการผู้ใช้งาน</h1>
        <div className="users-grid">
          {users.map((user) => (
            <div key={user.id} className="user-card">
              <div className="user-info">
                <h3>{user.first_name} {user.last_name}</h3>
                <p className="username">@{user.username}</p>
                <p className="created-date">
                  สมัครเมื่อ: {new Date(user.created_at).toLocaleDateString('th-TH')}
                </p>
              </div>
            </div>
          ))}
        </div>
        {users.length === 0 && (
          <div className="empty-state">ไม่มีข้อมูลผู้ใช้งาน</div>
        )}
      </div>
    </StyledWrapper>
  );
};

const StyledWrapper = styled.div`
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    background-color: #1a1a1a;
    min-height: 100vh;
  }

  h1 {
    color: white;
    text-align: center;
    margin-bottom: 30px;
    font-size: 2rem;
  }

  .users-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;
    margin-top: 20px;
  }

  .user-card {
    background: linear-gradient(135deg, #2a2a2a, #1a1a1a);
    border-radius: 15px;
    padding: 20px;
    border: 1px solid #333;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
  }

  .user-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.4);
    border-color: #555;
  }

  .user-info h3 {
    color: #fff;
    margin: 0 0 10px 0;
    font-size: 1.2rem;
  }

  .username {
    color: #888;
    margin: 5px 0;
    font-size: 0.9rem;
  }

  .created-date {
    color: #aaa;
    margin: 5px 0 0 0;
    font-size: 0.8rem;
  }

  .loading, .error, .empty-state {
    text-align: center;
    color: white;
    font-size: 1.2rem;
    margin-top: 50px;
  }

  .error {
    color: #ff6b6b;
  }

  .empty-state {
    color: #888;
  }
`;

export default UsersList;
