import { useEffect, useRef } from 'react';
import { Render } from 'skin3d';

export default function SkinViewer() {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const frameRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        const frame = frameRef.current;
        if (!canvas || !frame) return;

        const viewer = new Render({
            canvas,
            width: frame.clientWidth || 220,
            height: frame.clientHeight || 293,
        });
        viewer.loadSkin(`${import.meta.env.BASE_URL}assets/img/skin.png`);

        const resizeObserver = new ResizeObserver(([entry]) => {
            if (!entry || entry.contentRect.width === 0 || entry.contentRect.height === 0) return;
            viewer.width = entry.contentRect.width;
            viewer.height = entry.contentRect.height;
        });
        resizeObserver.observe(frame);
        
        return () => {
            resizeObserver.disconnect();
            viewer.dispose();
        };
    }, []);

    return (
        <div className="skin-canvas-frame" ref={frameRef}>
            <canvas ref={canvasRef} width={220} height={293} />
        </div>
    );
}