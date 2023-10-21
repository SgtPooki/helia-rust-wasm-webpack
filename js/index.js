import init, {bytes_to_string} from "../pkg/index.js"
import { createHelia } from 'helia'

try {
  console.log('bytes_to_string', bytes_to_string)
} catch(e) {
  console.error("Error importing `index.js`:", e)
}

const helia = createHelia()
window.helia = helia
console.log(`helia: `, helia);
