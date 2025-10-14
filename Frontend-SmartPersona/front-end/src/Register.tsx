import React, { useState, useCallback } from 'react';
import styled from 'styled-components';

interface RegisterProps {
  onSwitchToLogin?: () => void;
}

const Register = ({ onSwitchToLogin }: RegisterProps) => {
  const [formData, setFormData] = useState({
    username: '',
    firstname: '',
    lastname: '',
    password: '',
    confirmPassword: ''
  });
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const API_BASE = (import.meta as any).env?.VITE_BACKEND_URL || 'http://localhost:3000';

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

  const handleSubmit = useCallback(async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError(null);
    setSuccess(null);

    // Validation
    if (!formData.username || !formData.firstname || !formData.lastname || !formData.password) {
      setError('กรุณากรอกข้อมูลให้ครบถ้วน');
      return;
    }

    if (formData.password !== formData.confirmPassword) {
      setError('รหัสผ่านไม่ตรงกัน');
      return;
    }

    if (formData.password.length < 6) {
      setError('รหัสผ่านต้องมีอย่างน้อย 6 ตัวอักษร');
      return;
    }

    setIsSubmitting(true);
    try {
      const response = await fetch(`${API_BASE}/users`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          username: formData.username,
          firstname: formData.firstname,
          lastname: formData.lastname,
          password: formData.password,
        }),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || 'Registration failed');
      }

      setSuccess('สมัครสมาชิกสำเร็จ! กรุณาเข้าสู่ระบบ');
      // Reset form
      setFormData({
        username: '',
        firstname: '',
        lastname: '',
        password: '',
        confirmPassword: ''
      });
    } catch (err: any) {
      setError(err.message || 'เกิดข้อผิดพลาดในการสมัครสมาชิก');
    } finally {
      setIsSubmitting(false);
    }
  }, [API_BASE, formData]);

  return (
    <StyledWrapper>
      <form className="form" onSubmit={handleSubmit}>
        <p className="title">Register</p>
        <p className="message">Signup now and get full access to our app.</p>
        
        <div className="flex">
          <label>
            <input 
              required 
              placeholder=" " 
              type="text" 
              className="input" 
              name="firstname"
              value={formData.firstname}
              onChange={handleInputChange}
              disabled={isSubmitting}
            />
            <span>Firstname</span>
          </label>
          <label>
            <input 
              required 
              placeholder=" " 
              type="text" 
              className="input" 
              name="lastname"
              value={formData.lastname}
              onChange={handleInputChange}
              disabled={isSubmitting}
            />
            <span>Lastname</span>
          </label>
        </div>  
        
        <label>
          <input 
            required 
            placeholder=" " 
            type="text" 
            className="input" 
            name="username"
            value={formData.username}
            onChange={handleInputChange}
            disabled={isSubmitting}
          />
          <span>Username</span>
        </label> 
        
        <label>
          <input 
            required 
            placeholder=" " 
            type="password" 
            className="input" 
            name="password"
            value={formData.password}
            onChange={handleInputChange}
            disabled={isSubmitting}
          />
          <span>Password</span>
        </label>
        
        <label>
          <input 
            required 
            placeholder=" " 
            type="password" 
            className="input" 
            name="confirmPassword"
            value={formData.confirmPassword}
            onChange={handleInputChange}
            disabled={isSubmitting}
          />
          <span>Confirm password</span>
        </label>

        {error && <div className="error-message">{error}</div>}
        {success && <div className="success-message">{success}</div>}
        
        <button className="submit" type="submit" disabled={isSubmitting}>
          {isSubmitting ? 'กำลังสมัคร...' : 'Submit'}
        </button>
        
        <p className="signin">
          Already have an account? 
          <a href="#" onClick={(e) => { e.preventDefault(); onSwitchToLogin?.(); }}>
            Signin
          </a>
        </p>
      </form>
    </StyledWrapper>
  );
}

const StyledWrapper = styled.div`
  .form {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-width: 350px;
    background-color: #fff;
    padding: 20px;
    border-radius: 20px;
    position: relative;
  }

  .title {
    font-size: 28px;
    color: royalblue;
    font-weight: 600;
    letter-spacing: -1px;
    position: relative;
    display: flex;
    align-items: center;
    padding-left: 30px;
  }

  .title::before,.title::after {
    position: absolute;
    content: "";
    height: 16px;
    width: 16px;
    border-radius: 50%;
    left: 0px;
    background-color: royalblue;
  }

  .title::before {
    width: 18px;
    height: 18px;
    background-color: royalblue;
  }

  .title::after {
    width: 18px;
    height: 18px;
    animation: pulse 1s linear infinite;
  }

  .message, .signin {
    color: rgba(88, 87, 87, 0.822);
    font-size: 14px;
  }

  .signin {
    text-align: center;
  }

  .signin a {
    color: royalblue;
  }

  .signin a:hover {
    text-decoration: underline royalblue;
  }

  .flex {
    display: flex;
    width: 100%;
    gap: 6px;
  }

  .form label {
    position: relative;
  }

  .form label .input {
    width: 100%;
    padding: 10px 10px 20px 10px;
    outline: 0;
    border: 1px solid rgba(105, 105, 105, 0.397);
    border-radius: 10px;
  }

  .form label .input + span {
    position: absolute;
    left: 10px;
    top: 15px;
    color: grey;
    font-size: 0.9em;
    cursor: text;
    transition: 0.3s ease;
  }

  .form label .input:placeholder-shown + span {
    top: 15px;
    font-size: 0.9em;
  }

  .form label .input:focus + span,.form label .input:valid + span {
    top: 30px;
    font-size: 0.7em;
    font-weight: 600;
  }

  .form label .input:valid + span {
    color: green;
  }

  .submit {
    border: none;
    outline: none;
    background-color: royalblue;
    padding: 10px;
    border-radius: 10px;
    color: #fff;
    font-size: 16px;
    transform: .3s ease;
  }

  .submit:hover {
    background-color: rgb(56, 90, 194);
  }

  .submit:disabled {
    background-color: #ccc;
    cursor: not-allowed;
  }

  .error-message {
    color: #ff6b6b;
    font-size: 14px;
    text-align: center;
    margin: 10px 0;
  }

  .success-message {
    color: #51cf66;
    font-size: 14px;
    text-align: center;
    margin: 10px 0;
  }

  @keyframes pulse {
    from {
      transform: scale(0.9);
      opacity: 1;
    }

    to {
      transform: scale(1.8);
      opacity: 0;
    }
  }`;

export default Register;
