import React, { Dispatch, SetStateAction, useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api";

const FileViewer = () => {
  const [currentDirName, setCurrentDirName]: [string, Dispatch<SetStateAction<string>>] = useState("");
  const [index, setIndex]: [number, Dispatch<SetStateAction<number>>] = useState(0);
  const [filePaths, setFilePaths] = useState<string[]>([]);

  const [hopPath, setHopPath]: [string, Dispatch<SetStateAction<string>>] = useState("");

  const checkFiles = async () => {

    const files = await invoke("dirreader_api") as string[];

    setFilePaths([...files]);
  }

  const jumpToDir = async(dir: string) => {
    await invoke("hop_dir_api", {dir: dir});
  }

  const jumpToDirFinnish = async(dir: string) => {
    await jumpToDir(dir);
    checkFiles();
    fetchCurrentDirName();
  }

  const fetchCurrentDirName = async() => {
    const dirName = await invoke("display_current_dir_api") as string;

    setCurrentDirName(dirName);
  }

  const prevDir = async () => {
    await invoke("parent_dir_api");
  }

  useEffect(() => {
    checkFiles();
    fetchCurrentDirName();
  });
  return (
    <div>
      FileViewer
      <br />
      <h1>Current Directory</h1>
      <h2>{currentDirName}</h2>
      <br />
      <button onClick={prevDir}>Prev dir</button>
      <br />
      <button >JumpBack 1 step</button>

      {filePaths.map(filePath => {
        let filePathSplit = filePath.split("/");
        let filePathIndex = filePathSplit.length;
        return (
          <div>
            {filePathSplit[filePathIndex-1]}
            <button onClick={() => jumpToDirFinnish(filePath)}>chose directory</button>
          </div>
        );
      })}
    </div>
  )
}

export default FileViewer