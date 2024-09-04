import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";

export default function Window() {
    return (
        <Box>
            <AppBar data-tauri-drag-region="true">
                <div></div>
                <Toolbar>
                </Toolbar>
            </AppBar>
            <Box>
                <Toolbar />
            </Box>
        </Box>
    );
}
