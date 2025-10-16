import { useState, useRef, useEffect, useCallback } from 'react';
import { type KeyboardEvent } from 'react';

interface Message {
  id: string;
  content: string;
  role: 'user' | 'assistant';
  timestamp: Date;
}

const SendIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    fill="currentColor"
    width="20"
    height="20"
  >
    <path d="M22 2L11 13" />
    <path d="M22 2l-7 20-4-9-9-4 20-7z" />
  </svg>
);

const TypingIndicator = () => {
  return (
    <div className="flex items-center gap-1 px-4 py-3 bg-gradient-to-br from-slate-700/50 to-slate-600/50 rounded-2xl max-w-[80px] backdrop-blur-sm">
      {[0, 1, 2].map((dot) => (
        <div key={dot} className="w-2 h-2 bg-slate-300 rounded-full" />
      ))}
    </div>
  );
};

const ChatUI = () => {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: '1',
      content: "Hi! How can I help you today?",
      role: 'assistant',
      timestamp: new Date(),
    },
    {
      id: '2',
      content: "I'd like to learn about TypeScript and React.",
      role: 'user',
      timestamp: new Date(),
    },
    {
      id: '3',
      content: "I'd be happy to help you learn TypeScript and React! They're a powerful combination for building modern web applications. TypeScript adds static typing to JavaScript, which helps catch errors early and improves code quality. React is a popular library for building user interfaces with reusable components.\n\nWhat specific aspect would you like to start with?",
      role: 'assistant',
      timestamp: new Date(),
    },
  ]);
  const [inputValue, setInputValue] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  // biome-ignore lint/correctness/useExhaustiveDependencies: Need to scroll when messages or isTyping changes
  useEffect(() => {
    scrollToBottom();
  }, [messages, isTyping]);

  const handleSendMessage = () => {
    if (inputValue.trim() === '') return;

    const newMessage: Message = {
      id: Date.now().toString(),
      content: inputValue,
      role: 'user',
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, newMessage]);
    setInputValue('');
    setIsTyping(true);

    // Simulate AI response
    setTimeout(() => {
      const aiResponse: Message = {
        id: (Date.now() + 1).toString(),
        content: "That's a great question! I'm here to help you with that.",
        role: 'assistant',
        timestamp: new Date(),
      };
      setMessages((prev) => [...prev, aiResponse]);
      setIsTyping(false);
    }, 2000);
  };

  const handleKeyPress = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div className="flex flex-col h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
      {/* Header */}
      <div className="flex-shrink-0 bg-slate-800/50 backdrop-blur-sm border-b border-slate-700/50 px-6 py-4">
        <h1 className="text-2xl font-semibold text-white">AI Assistant</h1>
        <p className="text-sm text-slate-400 mt-1">Your intelligent companion</p>
      </div>

      {/* Chat Messages Area */}
      <div className="flex-1 overflow-y-auto px-4 py-6 space-y-6 scrollbar-thin scrollbar-thumb-slate-700 scrollbar-track-transparent">
        {messages.map((message) => (
          <div
            key={message.id}
            className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-[85%] md:max-w-[70%] lg:max-w-[60%] rounded-2xl px-5 py-3 shadow-lg ${
                message.role === 'user'
                  ? 'bg-gradient-to-br from-blue-600 to-blue-700 text-white'
                  : 'bg-gradient-to-br from-slate-700/50 to-slate-600/50 text-slate-100 backdrop-blur-sm'
              }`}
            >
              <p className="text-[15px] leading-relaxed whitespace-pre-wrap break-words">
                {message.content}
              </p>
              <span className="text-xs opacity-60 mt-2 block">
                {message.timestamp.toLocaleTimeString([], {
                  hour: '2-digit',
                  minute: '2-digit'
                })}
              </span>
            </div>
          </div>
        ))}

        {/* Typing Indicator */}
        {isTyping && (
          <div className="flex justify-start">
            <TypingIndicator />
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Input Area */}
      <div className="flex-shrink-0 bg-slate-800/30 backdrop-blur-sm border-t border-slate-700/50 px-4 py-4 md:px-6 md:py-6">
        <div className="max-w-4xl mx-auto">
          <div className="flex items-end gap-3 bg-slate-700/30 rounded-3xl px-5 py-3 shadow-xl border border-slate-600/30 focus-within:border-blue-500/50 transition-all duration-300">
            <input
              ref={inputRef}
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Type your message..."
              className="flex-1 bg-transparent text-white placeholder-slate-400 outline-none text-[15px] resize-none py-2"
              disabled={isTyping}
            />
            <button
              onClick={handleSendMessage}
              disabled={isTyping || inputValue.trim() === ''}
              className={`flex-shrink-0 p-3 rounded-full transition-all duration-200 ${
                isTyping || inputValue.trim() === ''
                  ? 'bg-slate-600/50 text-slate-400 cursor-not-allowed'
                  : 'bg-gradient-to-br from-blue-600 to-blue-700 text-white hover:from-blue-500 hover:to-blue-600 shadow-lg hover:shadow-blue-500/30'
              }`}
              aria-label="Send message"
            >
              <SendIcon />
            </button>
          </div>
          <p className="text-xs text-slate-500 text-center mt-3">
            Press Enter to send • Shift+Enter for new line
          </p>
        </div>
      </div>
    </div>
  );
};

export default ChatUI;
