
import { default as Schemas } from './schemas.test';

const Tests = {
    Schemas
}
const suites = Object.keys(Tests);

export enum TestPlatform {
    BROWSER = "BROWSER",
    NODE = "NODE"
}
export async function  runTests(platforms:TestPlatform[]): Promise<void> {
    platforms.forEach(platform => {
        if (platform === TestPlatform.BROWSER) {
            /**
             * @jest-environment jsdom
             */
        } else {
            /**
             * @jest-environment node
             */
        }
        suites.forEach(suiteName => {
            const suite = Tests[suiteName];
            suite(platform)
        })
    })
}
