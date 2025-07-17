// frontend/src/lib/themes.ts

export interface Theme {
  name: string;
  colors: {
    background: string;
    foreground: string;
    primary: string;
    'primary-foreground': string;
    secondary: string;
    'secondary-foreground': string;
    muted: string;
    'muted-foreground': string;
    accent: string;
    'accent-foreground': string;
    border: string;
    input: string;
    ring: string;
  };
}

export const themes: Theme[] = [
  {
    name: 'light',
    colors: {
      background: '0 0% 100%',
      foreground: '240 10% 3.9%',
      primary: '240 5.9% 10%',
      'primary-foreground': '0 0% 98%',
      secondary: '240 4.8% 95.9%',
      'secondary-foreground': '240 5.9% 10%',
      muted: '240 4.8% 95.9%',
      'muted-foreground': '240 3.8% 46.1%',
      accent: '240 4.8% 95.9%',
      'accent-foreground': '240 5.9% 10%',
      border: '240 5.9% 90%',
      input: '240 5.9% 90%',
      ring: '240 5.9% 10%',
    },
  },
  {
    name: 'dark',
    colors: {
      background: '240 10% 3.9%',
      foreground: '0 0% 98%',
      primary: '0 0% 98%',
      'primary-foreground': '240 5.9% 10%',
      secondary: '240 3.7% 15.9%',
      'secondary-foreground': '0 0% 98%',
      muted: '240 3.7% 15.9%',
      'muted-foreground': '240 5% 64.9%',
      accent: '240 3.7% 15.9%',
      'accent-foreground': '0 0% 98%',
      border: '240 3.7% 15.9%',
      input: '240 3.7% 15.9%',
      ring: '240 4.9% 83.9%',
    },
  },
  {
    name: 'rose-pine',
    colors: {
      background: '259 26% 14%',
      foreground: '259 26% 90%',
      primary: '259 26% 70%',
      'primary-foreground': '259 26% 10%',
      secondary: '259 26% 20%',
      'secondary-foreground': '259 26% 90%',
      muted: '259 26% 20%',
      'muted-foreground': '259 26% 60%',
      accent: '259 26% 25%',
      'accent-foreground': '259 26% 90%',
      border: '259 26% 25%',
      input: '259 26% 25%',
      ring: '259 26% 70%',
    },
  },
  {
    name: 'solarized-light',
    colors: {
      background: '44 23% 94%',
      foreground: '195 15% 41%',
      primary: '195 15% 41%',
      'primary-foreground': '44 23% 94%',
      secondary: '44 23% 89%',
      'secondary-foreground': '195 15% 41%',
      muted: '44 23% 89%',
      'muted-foreground': '195 15% 55%',
      accent: '44 23% 89%',
      'accent-foreground': '195 15% 41%',
      border: '44 23% 84%',
      input: '44 23% 84%',
      ring: '195 15% 41%',
    },
  },
  {
    name: 'solarized-dark',
    colors: {
      background: '195 23% 7%',
      foreground: '195 15% 65%',
      primary: '195 15% 65%',
      'primary-foreground': '195 23% 7%',
      secondary: '195 23% 11%',
      'secondary-foreground': '195 15% 65%',
      muted: '195 23% 11%',
      'muted-foreground': '195 15% 55%',
      accent: '195 23% 15%',
      'accent-foreground': '195 15% 65%',
      border: '195 23% 15%',
      input: '195 23% 15%',
      ring: '195 15% 65%',
    },
  },
  {
    name: 'dracula',
    colors: {
      background: '231 15% 18%',
      foreground: '60 30% 96%',
      primary: '265 89% 78%',
      'primary-foreground': '231 15% 18%',
      secondary: '231 15% 24%',
      'secondary-foreground': '60 30% 96%',
      muted: '231 15% 24%',
      'muted-foreground': '231 15% 62%',
      accent: '231 15% 29%',
      'accent-foreground': '60 30% 96%',
      border: '231 15% 29%',
      input: '231 15% 29%',
      ring: '265 89% 78%',
    },
  },
];
