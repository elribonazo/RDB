import { Buffer } from 'buffer';

if (typeof window !== 'undefined') {
    (window as any).Buffer = Buffer;
} else {
    global.Buffer = Buffer;
}
