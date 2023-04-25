import React, {Dispatch, SetStateAction, useState, useEffect} from 'react'
import { invoke } from '@tauri-apps/api';
import FileViewer from './Components/FileViewer';
import Player from './Components/Player';

const App = () => {
  const [greetMessage, setGreetMessage]: [string, Dispatch<SetStateAction<string>>] = useState("");
  const [selectedFileBool, setSelectedFileBool]: [boolean, Dispatch<SetStateAction<boolean>>] = useState(false);

  const fetchGreeting = async() => {
    const invocation = await invoke("greeting_api");
    setGreetMessage(invocation as string);
  }

  const handleSelect = (selectedFileBool: boolean) => {
    setSelectedFileBool(!selectedFileBool);
  }

  useEffect(() => {
    fetchGreeting();
  }, [selectedFileBool]);

  return (
    <div>
      App
      <br />
      {greetMessage}

      <br />
      {selectedFileBool ? <Player selectState={selectedFileBool} selectStateChange={handleSelect} /> : <FileViewer selectState={selectedFileBool} selectStateChange={handleSelect} />}
    </div>
  )
}

export default App