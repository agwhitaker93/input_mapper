import { useState, useEffect } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'

export function Device() {
    const [devices, setDevices] = useState([])

    useEffect(() => {
        invoke('get_devices').then((res) => {
            setDevices(res)
        })
    }, [])

    function deviceChange(e) {
        invoke('select_device', {idx: parseInt(e.target.value)})
    }

    return (
        <div>
          <h2>Select Device</h2>
          <select name="device" onChange={deviceChange}>
            <option value="">--Please select a device to listen to--</option>
            {devices.map(([id, name]) => <option value={id}>{name}</option>)}
          </select>
        </div>
    )
}
