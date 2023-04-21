import React, { Dispatch, SetStateAction, useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api";

const FileViewer = () => {
  const [currentDirName, setCurrentDirName]: [string, Dispatch<SetStateAction<string>>] = useState("");
  const [index, setIndex]: [number, Dispatch<SetStateAction<number>>] = useState(0);
  const [currentDir, setCurrentDir] = useState<string[]>(["./"]);
  const [filePaths, setFilePaths] = useState<string[]>([]);

  const [hopPath, setHopPath]: [string, Dispatch<SetStateAction<string>>] = useState("");

  const checkFiles = async () => {
    const dir = currentDir.join().replaceAll(",", "");

    const files = await invoke("dirreader_api", { dir: dir }) as string[];

    setFilePaths([...files]);
  }

  const browseDirs = () => {
    setCurrentDir([...currentDir, "../"]);
    setIndex(index + 1);
  }

  const jumpToPrevious = () => {
    if (currentDir.length === 1) {
      return;
    }

    const copyArr = [...currentDir]
    copyArr.pop();
    setCurrentDir(copyArr);
    setIndex(index - 1);
  }

  const jumpToDir = async(dir: string) => {
    await invoke("hop_dir_api", {dir: dir});
  }

  const jumpToDirFinnish = async(dir: string) => {
    await jumpToDir(dir);
    setHopPath("");
    checkFiles();

    for(let i = 0; i <= index; i++) {
      jumpToPrevious();
    }
    fetchCurrentDirName();
  }

  const fetchCurrentDirName = async() => {
    const dirName = await invoke("display_current_dir_api") as string;

    setCurrentDirName(dirName);
  }

  useEffect(() => {
    checkFiles();
  }, [currentDir]);
  return (
    <div>
      FileViewer
      <br />
      <h1>Current Directory</h1>
      <h2>{currentDirName}</h2>
      <br />
      <button onClick={browseDirs}>Prev dir</button>
      <br />
      <button onClick={jumpToPrevious}>JumpBack 1 step</button>

      {filePaths.map(filePath => {
        return (
          <div>
            {filePath}
            <button onClick={() => jumpToDirFinnish(filePath)}>chose directory</button>
          </div>
        );
      })}
    </div>
  )
}

export default FileViewer