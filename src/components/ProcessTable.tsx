import {useEffect, useState} from "react";
import {get_network_address_info} from "../utils/invoke";


interface NetWorkInfo {
    proto: string;
    local_port: string;
    remote_addr: string;
    remote_port: string;
    pid:string;
    state:string;
}

export default function ProcessTable() {

    const [NetWorkInfos,setNetWorkInfo] = useState<NetWorkInfo[]>([])
    useEffect(()=>{
        const intervalId = setInterval(()=>{
            get_network_address_info()
            .then((data:unknown)=>{
                const infos = data as NetWorkInfo[];
                setNetWorkInfo(infos);
                // console.log(infos)
            })
            .catch((err:Error)=>{
                console.error(err)
            })
        },1000)

        return ()=>{
            clearInterval(intervalId)
        }
    },[])

    //  useEffect(() => {
    //     console.log('Network info updated:', NetWorkInfos);
    // }, [NetWorkInfos]);

    return (
        <div className="overflow-y-scroll w-full h-full">
            <table className="table text-center overflow-y-scroll ">
                {/* head */}
                <thead className="sticky top-0 bg-gray-200 rounded-4xl  shadow-lg ">
                <tr  >
                    <th className="p-2">Index</th>
                    <th className="p-2">Proto</th>
                    <th className="p-2">Local Proto</th>
                    <th className="p-2">Remote Address</th>
                    <th className="p-2">Remote Port</th>
                    <th className="p-2">Pid</th>
                    <th className="p-2">State</th>
                </tr>
                </thead>
                <tbody>
                {
                    NetWorkInfos.map((item,index)=>( 
   
                        <tr key={index} className="hover:bg-gray-200"> 
                            <td>{index}</td>
                            <td>{item.proto}</td>
                            <td>{item.local_port}</td>
                            <td>{item.remote_addr}</td>
                            <td>{item.remote_port}</td>
                            <td>{item.pid}</td>
                            <td>{item.state}</td>
                        </tr>
                    ))
                }
                </tbody>
            </table>
    
        </div>
    );
}