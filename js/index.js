const main = async () => {
  try {

    const index = await import("../pkg/index.js")
    console.log(index)
    index()
  } catch(e) {
    console.error("Error importing `index.js`:", e)
  }
}

main() // Call main function
