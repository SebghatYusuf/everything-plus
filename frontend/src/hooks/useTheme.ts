import { useState, useEffect, useCallback } from 'react';
import { themes } from '../lib/themes';

export function useTheme() {
  const [themeName, setThemeName] = useState('dark');

  const applyTheme = useCallback((name: string) => {
    const newTheme = themes.find(t => t.name === name) || themes[0];
    const root = window.document.documentElement;

    // Remove all theme classes
    for (const t of themes) {
      root.classList.remove(t.name);
    }
    
    // Add new theme class
    root.classList.add(newTheme.name);

    // Apply CSS variables
    for (const [key, value] of Object.entries(newTheme.colors)) {
      root.style.setProperty(`--${key}`, value);
    }

    setThemeName(newTheme.name);
    localStorage.setItem('theme', newTheme.name);
  }, []);

  useEffect(() => {
    const savedTheme = localStorage.getItem('theme');
    if (savedTheme) {
      applyTheme(savedTheme);
    } else {
      // Default to system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      applyTheme(prefersDark ? 'dark' : 'light');
    }
  }, [applyTheme]);

  const toggleTheme = () => {
    const newThemeName = themeName === 'light' ? 'dark' : 'light';
    applyTheme(newThemeName);
  };

  return { theme: themeName, applyTheme, toggleTheme, themes };
}
