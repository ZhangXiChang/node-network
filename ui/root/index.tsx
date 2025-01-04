import { Checkbox } from "@ark-ui/solid";
import { CheckIcon } from "lucide-solid";

const items = [
    { label: "React", value: "react" },
    { label: "Solid", value: "solid" },
    { label: "Vue", value: "vue" },
];

export function Root() {
    return <Checkbox.Group defaultValue={["react"]} onValueChange={console.log}>
        {items.map((item) => (
            <Checkbox.Root value={item.value}>
                <Checkbox.Label>{item.label}</Checkbox.Label>
                <Checkbox.Control>
                    <Checkbox.Indicator>
                        <CheckIcon />
                    </Checkbox.Indicator>
                </Checkbox.Control>
                <Checkbox.HiddenInput />
            </Checkbox.Root>
        ))}
    </Checkbox.Group>;
}
