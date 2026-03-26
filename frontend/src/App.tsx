import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Item, Resumen } from './types';
import './index.css';

function App() {
  const [items, setItems] = useState<Item[]>([]);
  const [resumen, setResumen] = useState<Resumen | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [showForm, setShowForm] = useState<boolean>(false);
  const [searchTerm, setSearchTerm] = useState<string>('');

  const [producto, setProducto] = useState<string>('');
  const [cantidad, setCantidad] = useState<string>('1');
  const [precio, setPrecio] = useState<string>('');
  const [tienda, setTienda] = useState<string>('');
  const [categoria, setCategoria] = useState<string>('general');

  const loadData = async (): Promise<void> => {
    setLoading(true);
    try {
      const [itemsData, resumenData] = await Promise.all([
        invoke<Item[]>('get_items'),
        invoke<Resumen>('get_resumen')
      ]);
      setItems(itemsData);
      setResumen(resumenData);
    } catch (error) {
      console.error('Error loading data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const handleAddItem = async (e: React.FormEvent<HTMLFormElement>): Promise<void> => {
    e.preventDefault();
    try {
      const newItem = await invoke<Item>('add_item', {
        producto,
        cantidad_mensual: parseFloat(cantidad),
        precio_unitario: parseInt(precio),
        tienda,
        categoria
      });
      setItems([...items, newItem]);
      setShowForm(false);
      setProducto('');
      setCantidad('1');
      setPrecio('');
      setTienda('');
      setCategoria('general');
      loadData();
    } catch (error) {
      console.error('Error adding item:', error);
      alert('Error al agregar: ' + error);
    }
  };

  const handleDeleteItem = async (id: string): Promise<void> => {
    if (confirm('¿Eliminar este item?')) {
      try {
        await invoke('delete_item', { id });
        setItems(items.filter(item => item.id !== id));
        loadData();
      } catch (error) {
        console.error('Error deleting item:', error);
        alert('Error al eliminar: ' + error);
      }
    }
  };

  const formatCOP = (amount: number): string => {
    return new Intl.NumberFormat('es-CO', {
      style: 'currency',
      currency: 'COP',
      minimumFractionDigits: 0
    }).format(amount);
  };

  const filteredItems: Item[] = items.filter(item =>
      item.producto.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const styles = {
    container: {
      minHeight: '100vh',
      backgroundColor: '#f3f4f6',
      paddingBottom: '80px',
    },
    header: {
      backgroundColor: '#16a34a',
      color: 'white',
      padding: '16px',
      position: 'sticky' as const,
      top: 0,
      zIndex: 10,
      boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
    },
    headerTitle: {
      fontSize: '24px',
      fontWeight: 'bold' as const,
    },
    headerSub: {
      marginTop: '8px',
      fontSize: '14px',
    },
    main: {
      padding: '16px',
      maxWidth: '800px',
      margin: '0 auto',
    },
    card: {
      backgroundColor: 'white',
      borderRadius: '8px',
      padding: '16px',
      marginBottom: '16px',
      boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
    },
    grid2: {
      display: 'grid',
      gridTemplateColumns: '1fr 1fr',
      gap: '16px',
      marginBottom: '24px',
    },
    cardTitle: {
      fontSize: '14px',
      color: '#6b7280',
      marginBottom: '8px',
    },
    searchInput: {
      width: '100%',
      padding: '12px',
      border: '1px solid #d1d5db',
      borderRadius: '8px',
      marginBottom: '16px',
      fontSize: '16px',
    },
    item: {
      backgroundColor: 'white',
      borderRadius: '8px',
      padding: '16px',
      marginBottom: '12px',
      boxShadow: '0 1px 3px rgba(0,0,0,0.1)',
    },
    itemTitle: {
      fontSize: '18px',
      fontWeight: 'bold' as const,
    },
    itemBadge: {
      backgroundColor: '#f3f4f6',
      padding: '4px 8px',
      borderRadius: '4px',
      fontSize: '12px',
      marginRight: '8px',
    },
    fab: {
      position: 'fixed' as const,
      bottom: '24px',
      right: '24px',
      backgroundColor: '#16a34a',
      color: 'white',
      width: '56px',
      height: '56px',
      borderRadius: '50%',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      fontSize: '24px',
      cursor: 'pointer',
      boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
      border: 'none',
    },
    modal: {
      position: 'fixed' as const,
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      backgroundColor: 'rgba(0,0,0,0.5)',
      display: 'flex',
      alignItems: 'flex-end' as const,
      justifyContent: 'center',
      zIndex: 50,
    },
    modalContent: {
      backgroundColor: 'white',
      borderRadius: '16px 16px 0 0',
      width: '100%',
      maxWidth: '500px',
      padding: '24px',
    },
    input: {
      width: '100%',
      padding: '12px',
      border: '1px solid #d1d5db',
      borderRadius: '8px',
      marginBottom: '12px',
      fontSize: '16px',
    },
    button: {
      width: '100%',
      backgroundColor: '#16a34a',
      color: 'white',
      padding: '12px',
      border: 'none',
      borderRadius: '8px',
      fontSize: '16px',
      fontWeight: 'bold' as const,
      cursor: 'pointer',
    },
    deleteButton: {
      backgroundColor: 'transparent',
      border: 'none',
      fontSize: '20px',
      cursor: 'pointer',
      color: '#ef4444',
    },
  };

  return (
      <div style={styles.container}>
        <div style={styles.header}>
          <div style={styles.headerTitle}>🏠 Inventario Hogar</div>
          {resumen && (
              <div style={styles.headerSub}>
                Total mensual: {formatCOP(resumen.total_mensual)} | Items: {resumen.total_items}
              </div>
          )}
        </div>

        <div style={styles.main}>
          {resumen && (
              <div style={styles.grid2}>
                <div style={styles.card}>
                  <div style={styles.cardTitle}>Gasto por categoría</div>
                  {resumen.gasto_por_categoria.slice(0, 4).map(([cat, total]: [string, number]) => (
                      <div key={cat} style={{ display: 'flex', justifyContent: 'space-between', marginTop: '8px' }}>
                        <span>{cat}</span>
                        <span style={{ fontWeight: 'bold' }}>{formatCOP(total)}</span>
                      </div>
                  ))}
                </div>
                <div style={styles.card}>
                  <div style={styles.cardTitle}>Top items</div>
                  {resumen.top_items.slice(0, 3).map((item: Item) => (
                      <div key={item.id} style={{ display: 'flex', justifyContent: 'space-between', marginTop: '8px' }}>
                  <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', maxWidth: '120px' }}>
                    {item.producto}
                  </span>
                        <span style={{ fontWeight: 'bold' }}>{formatCOP(item.costo_mensual)}</span>
                      </div>
                  ))}
                </div>
              </div>
          )}

          <input
              type="text"
              placeholder="🔍 Buscar producto..."
              style={styles.searchInput}
              value={searchTerm}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setSearchTerm(e.target.value)}
          />

          {loading ? (
              <div style={{ textAlign: 'center', padding: '32px' }}>Cargando...</div>
          ) : filteredItems.length === 0 ? (
              <div style={{ textAlign: 'center', padding: '32px', color: '#6b7280' }}>No hay items</div>
          ) : (
              filteredItems.map((item: Item) => (
                  <div key={item.id} style={styles.item}>
                    <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                      <div style={{ flex: 1 }}>
                        <div style={styles.itemTitle}>{item.producto}</div>
                        <div style={{ marginTop: '4px' }}>
                          <span style={styles.itemBadge}>{item.categoria}</span>
                          <span style={{ marginLeft: '8px' }}>🏪 {item.tienda}</span>
                        </div>
                        <div style={{ marginTop: '8px', display: 'flex', gap: '12px', fontSize: '14px' }}>
                          <span>📦 {item.cantidad_mensual} ud/mes</span>
                          <span>💰 {formatCOP(item.precio_unitario)} c/u</span>
                          <span style={{ color: '#16a34a', fontWeight: 'bold' }}>{formatCOP(item.costo_mensual)}/mes</span>
                        </div>
                      </div>
                      <button
                          onClick={() => handleDeleteItem(item.id)}
                          style={styles.deleteButton}
                      >
                        🗑️
                      </button>
                    </div>
                  </div>
              ))
          )}
        </div>

        <button style={styles.fab} onClick={() => setShowForm(true)}>
          +
        </button>

        {showForm && (
            <div style={styles.modal} onClick={() => setShowForm(false)}>
              <div style={styles.modalContent} onClick={(e: React.MouseEvent) => e.stopPropagation()}>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '16px' }}>
                  <h2 style={{ fontSize: '20px', fontWeight: 'bold' }}>Agregar Producto</h2>
                  <button onClick={() => setShowForm(false)} style={{ background: 'none', border: 'none', fontSize: '20px' }}>✕</button>
                </div>
                <form onSubmit={handleAddItem}>
                  <input
                      type="text"
                      placeholder="Producto"
                      style={styles.input}
                      value={producto}
                      onChange={(e: React.ChangeEvent<HTMLInputElement>) => setProducto(e.target.value)}
                      required
                  />
                  <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
                    <input
                        type="number"
                        step="0.01"
                        placeholder="Cantidad/mes"
                        style={styles.input}
                        value={cantidad}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => setCantidad(e.target.value)}
                        required
                    />
                    <input
                        type="number"
                        placeholder="Precio unitario"
                        style={styles.input}
                        value={precio}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPrecio(e.target.value)}
                        required
                    />
                  </div>
                  <input
                      type="text"
                      placeholder="Tienda"
                      style={styles.input}
                      value={tienda}
                      onChange={(e: React.ChangeEvent<HTMLInputElement>) => setTienda(e.target.value)}
                      required
                  />
                  <select
                      style={styles.input}
                      value={categoria}
                      onChange={(e: React.ChangeEvent<HTMLSelectElement>) => setCategoria(e.target.value)}
                  >
                    <option value="general">General</option>
                    <option value="higiene">Higiene</option>
                    <option value="hogar">Hogar</option>
                    <option value="limpieza">Limpieza</option>
                    <option value="granos">Granos</option>
                  </select>
                  <button type="submit" style={styles.button}>
                    Agregar
                  </button>
                </form>
              </div>
            </div>
        )}
      </div>
  );
}

export default App;