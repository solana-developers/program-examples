import { useEffect, useState } from "react"
import { buildStyles, CircularProgressbar } from "react-circular-progressbar"
import "react-circular-progressbar/dist/styles.css"

const Confirmed = () => {
  const [percentage, setPercentage] = useState(0)
  const [text, setText] = useState("😋")

  useEffect(() => {
    const t1 = setTimeout(() => setPercentage(100), 100)
    const t2 = setTimeout(() => setText("✅"), 600)

    return () => {
      clearTimeout(t1)
      clearTimeout(t2)
    }
  }, [])

  return (
    <CircularProgressbar
      value={percentage}
      text={text}
      styles={buildStyles({
        pathColor: "#19fb9b",
      })}
    />
  )
}

export default Confirmed
