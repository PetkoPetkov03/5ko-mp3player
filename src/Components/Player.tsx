import { Props } from "../interfaces/main"

const Player = ({selectState, selectStateChange} : Props) => {

    const goBack = () => {
        selectStateChange(selectState);
    }

  return (
    <div>
        <button onClick={goBack}>Back</button>
        Player
    </div>
  )
}

export default Player