import React, { Dispatch, SetStateAction, useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api";
import { Props } from '../interfaces/main';

const FileViewer = ({ selectState, selectStateChange }: Props) => {

  enum FileType {
    FOLDER = "FOLDER",
    MP3 = "MP3"
  }

  interface FileStruct {
    file_path: string,
    file_type: FileType
  }

  const [currentDirName, setCurrentDirName]: [string, Dispatch<SetStateAction<string>>] = useState("");
  const [index, setIndex]: [number, Dispatch<SetStateAction<number>>] = useState(0);
  const [filePaths, setFilePaths] = useState<FileStruct[]>([]);

  const [hopPath, setHopPath]: [string, Dispatch<SetStateAction<string>>] = useState("");

  const checkFiles = async () => {

    const files = await invoke("dirreader_api") as FileStruct[];
    setFilePaths([...files]);

    fetchCurrentDirName();
  }

  const jumpToDir = async(dir: string) => {
    await invoke("hop_dir_api", {dir: dir});
    fetchCurrentDirName();
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
    fetchCurrentDirName();
  }

  const previousAction = async() => {
    await invoke("previous_action_api");
    fetchCurrentDirName();
  }

  const selectFile = () => {  
    selectStateChange(selectState)
  }

  useEffect(() => {
    checkFiles();
  }, [currentDirName]);

  return (
    <div>
      FileViewer
      <br />
      <h1>Current Directory</h1>
      <h2>{currentDirName}</h2>
      <br />
      <button onClick={prevDir}>Prev dir</button>
      <br />
      <button onClick={previousAction}>JumpBack 1 step</button>

      {filePaths.map(filePath => {
        
        let filePathSplit = filePath.file_path.split("/");
        let filePathIndex = filePathSplit.length;
        return (
          <div>
            {filePathSplit[filePathIndex-1]}
            { filePath.file_type === FileType.FOLDER ? <button onClick={() => jumpToDirFinnish(filePath.file_path)}>chose directory</button> : <button onClick={selectFile}>Play file</button>}
          </div>
        );
      })}
    </div>
  )
}

export default FileViewer