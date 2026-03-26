export interface Item {
    id: string;
    producto: string;
    cantidad_mensual: number;
    precio_unitario: number;
    costo_mensual: number;
    tienda: string;
    categoria: string;
}

export interface Resumen {
    total_mensual: number;
    total_items: number;
    gasto_por_categoria: [string, number][];
    top_items: Item[];
}