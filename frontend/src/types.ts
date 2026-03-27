export interface Item {
    id: string;
    producto: string;
    cantidadMensual: number;      // ← camelCase
    precioUnitario: number;        // ← camelCase
    costoMensual: number;
    tienda: string;
    categoria: string;
}

export interface Resumen {
    total_mensual: number;
    total_items: number;
    gasto_por_categoria: [string, number][];
    top_items: Item[];
}