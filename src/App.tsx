import React, {Dispatch, SetStateAction, useState, useEffect} from 'react'
import { invoke } from '@tauri-apps/api';
import FileViewer from './Components/FileViewer';

const App = () => {
  const [greetMessage, setGreetMessage]: [string, Dispatch<SetStateAction<string>>] = useState("");

  const fetchGreeting = async() => {
    const invocation = await invoke("greeting_api");
    setGreetMessage(invocation as string);
  }

  useEffect(() => {
    fetchGreeting();
  }, []);

  return (
    <div>
      App
      <br />
      {greetMessage}

      <br />
      <FileViewer />
    </div>
  )
}

export default App