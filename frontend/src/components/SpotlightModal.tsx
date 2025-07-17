import React, { useState, useEffect, useRef } from 'react';
import { useSearch } from '../hooks/useSearch';
import { TauriAPI } from '../lib/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { FileText, Search, File, Folder, Calculator, Globe } from 'lucide-react';
import { getFileIcon, highlightText } from '../lib/utils';
import { Button } from './ui/button';

type FilterType = 'all' | 'files' | 'folders';

const SpotlightModal: React.FC = () => {
  const { query, setQuery, results, isLoading, setFilters } = useSearch();
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [activeFilter, setActiveFilter] = useState<FilterType>('all');
  const [calculatorResult, setCalculatorResult] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const resultsRef = useRef<HTMLUListElement>(null);

  // Basic calculator functionality
  useEffect(() => {
    const calculate = (expr: string): string | null => {
      try {
        // Very basic and unsafe eval. In a real app, use a proper math parser.
        if (/^[\d\s()+\-*\/.^%]+$/.test(expr)) {
          const result = new Function(`return ${expr}`)();
          if (typeof result === 'number' && !isNaN(result)) {
            return result.toLocaleString(); // Format with commas
          }
        }
      } catch (error) {
        // Not a valid expression
      }
      return null;
    };

    const result = calculate(query);
    setCalculatorResult(result);
  }, [query]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedIndex((prev) => Math.min(prev + 1, results.length - 1));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedIndex((prev) => Math.max(prev - 1, 0));
      } else if (e.key === 'Enter') {
        e.preventDefault();
        if (calculatorResult) {
          navigator.clipboard.writeText(calculatorResult);
          appWindow.hide();
          return;
        }
        if (results.length > 0) {
          const selectedItem = results[selectedIndex];
          if (selectedItem.type === 'url') {
            TauriAPI.openLink(selectedItem.path);
          } else if (e.ctrlKey || e.metaKey) {
            TauriAPI.openFileLocation(selectedItem.path);
          } else {
            TauriAPI.openFile(selectedItem.path);
          }
          appWindow.hide();
        } else {
          // If no results, check if the query is a URL
          const urlRegex = /^(https?:\/\/)?([\da-z.-]+)\.([a-z.]{2,6})([/\w .-]*)*\/?$/;
          if (urlRegex.test(query)) {
            TauriAPI.openLink(query.startsWith('http') ? query : `https://${query}`);
            appWindow.hide();
          }
        }
      } else if (e.key === 'Escape') {
        appWindow.hide();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [results, selectedIndex, calculatorResult]);

  useEffect(() => {
    inputRef.current?.focus();
    const unlisten = appWindow.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        setQuery('');
        setActiveFilter('all');
        inputRef.current?.focus();
      }
    });
    return () => {
      unlisten.then(f => f());
    };
  }, [setQuery]);

  useEffect(() => {
    if (resultsRef.current && results.length > 0) {
      const selectedElement = resultsRef.current.children[selectedIndex] as HTMLLIElement;
      if (selectedElement) {
        selectedElement.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
      }
    }
  }, [selectedIndex, results]);

  useEffect(() => {
    setFilters(prev => ({
      ...prev,
      filesOnly: activeFilter === 'files',
      directoriesOnly: activeFilter === 'folders',
    }));
  }, [activeFilter, setFilters]);

  return (
    <div 
      className="bg-background/80 backdrop-blur-sm rounded-lg shadow-2xl w-full h-full flex flex-col overflow-hidden animate-scale-in"
      onMouseDown={(e) => {
        // Allow dragging only on the top bar
        if (e.target === e.currentTarget) {
          e.preventDefault();
          appWindow.startDragging();
        }
      }}
    >
      <div className="flex items-center gap-3 p-4 border-b border-border">
        <Search className="w-6 h-6 text-muted-foreground" />
        <input
          ref={inputRef}
          type="text"
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setSelectedIndex(0);
          }}
          placeholder="Search files, folders, or calculate..."
          className="w-full bg-transparent text-foreground text-xl placeholder-muted-foreground focus:outline-none"
        />
        {isLoading && <div className="w-5 h-5 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>}
      </div>

      {calculatorResult && (
        <div className="p-4 border-b border-border">
          <div className="flex items-center gap-3 text-2xl">
            <Calculator className="w-6 h-6 text-muted-foreground" />
            <span className="font-semibold">{calculatorResult}</span>
          </div>
          <div className="text-xs text-muted-foreground ml-9">Press Enter to copy</div>
        </div>
      )}

      {!calculatorResult && (
        <>
          <div className="p-2 border-b border-border flex items-center gap-2">
            <Button variant={activeFilter === 'all' ? 'secondary' : 'ghost'} size="sm" onClick={() => setActiveFilter('all')}>All</Button>
            <Button variant={activeFilter === 'files' ? 'secondary' : 'ghost'} size="sm" onClick={() => setActiveFilter('files')}><File className="w-4 h-4 mr-2" />Files</Button>
            <Button variant={activeFilter === 'folders' ? 'secondary' : 'ghost'} size="sm" onClick={() => setActiveFilter('folders')}><Folder className="w-4 h-4 mr-2" />Folders</Button>
          </div>

          <div className="flex-grow overflow-y-auto">
            {!isLoading && results.length === 0 && query.trim() && (
              <div className="text-center py-12 text-muted-foreground">
                <FileText className="w-12 h-12 mx-auto mb-4" />
                <h3 className="text-lg font-medium">No results found</h3>
                <p>Try a different search term or filter.</p>
              </div>
            )}
            
            {!isLoading && (
              <ul ref={resultsRef} className="p-2">
                {results.map((item, index) => {
                  const isUrl = item.type === 'url';
                  return (
                    <li
                      key={item.path}
                      className={`flex items-center gap-3 p-3 rounded-md cursor-pointer transition-colors ${
                        selectedIndex === index
                          ? 'bg-primary text-primary-foreground'
                          : 'hover:bg-accent/50'
                      }`}
                      onClick={() => isUrl ? TauriAPI.openLink(item.path) : TauriAPI.openFile(item.path)}
                      onMouseEnter={() => setSelectedIndex(index)}
                      onDoubleClick={() => isUrl ? TauriAPI.openLink(item.path) : TauriAPI.openFileLocation(item.path)}
                      title={`Click to open • Double-click to show in folder\n${item.path}`}
                    >
                      <div className="text-2xl w-8 text-center">
                        {isUrl ? <Globe className="w-6 h-6" /> : getFileIcon(item.extension, item.type === 'folder')}
                      </div>
                      <div className="flex-1 min-w-0">
                        <p
                          className="font-medium truncate"
                          dangerouslySetInnerHTML={{ __html: highlightText(item.name, query) }}
                        />
                        <p className={`text-sm truncate ${
                          selectedIndex === index ? 'text-primary-foreground/80' : 'text-muted-foreground'
                        }`}>
                          {item.path}
                        </p>
                      </div>
                    </li>
                  );
                })}
              </ul>
            )}
          </div>
        </>
      )}
      
      <div className="p-2 border-t border-border text-xs text-muted-foreground text-center">
        Use <kbd className="px-1.5 py-0.5 border rounded bg-muted">↑</kbd> <kbd className="px-1.5 py-0.5 border rounded bg-muted">↓</kbd> to navigate, <kbd className="px-1.5 py-0.5 border rounded bg-muted">Enter</kbd> to open.
      </div>
    </div>
  );
};

export default SpotlightModal;

