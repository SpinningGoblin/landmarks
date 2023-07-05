import { FC, useState } from "react";
import { useParams } from "react-router-dom";
import { useWorld } from "../hooks/worlds";
import {
  Box,
  Button,
  Chip,
  FormControl,
  InputLabel,
  MenuItem,
  Paper,
  Select,
  SelectChangeEvent,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { NumberField } from "../components/NumberField";
import { TextListEditor } from "../components/TextListEditor";
import { useBiomes, useDimensions } from "../hooks/minecraft";
import { useAddLandmark } from "../hooks/landmarks";
import { CreateLandmark } from "../api/CreateLandmark";

export const AddLandmark: FC = () => {
  const { worldId } = useParams();
  const { world, isLoading } = useWorld(worldId);
  const { biomes, isLoading: isBiomesLoading } = useBiomes();
  const { dimensions, isLoading: isDimensionsLoading } = useDimensions();
  const [name, setName] = useState("");
  const [x, setX] = useState("");
  const [y, setY] = useState("");
  const [z, setZ] = useState("");
  const [tags, setTags] = useState<string[]>([]);
  const [farms, setFarms] = useState<string[]>([]);
  const [dimension, setDimension] = useState("");
  const [selectedBiomes, setSelectedBiomes] = useState<string[]>([]);
  const [notes, setNotes] = useState("");
  const { addLandmark } = useAddLandmark(worldId);

  const handleDimensionChange = (event: SelectChangeEvent) => {
    setDimension(event.target.value as string);
  };

  const handleBiomeChange = (
    event: SelectChangeEvent<typeof selectedBiomes>,
  ) => {
    const {
      target: { value },
    } = event;
    setSelectedBiomes(
      // On autofill we get a stringified value.
      typeof value === "string" ? value.split(",") : value,
    );
  };

  const handleSubmitLandmark = () => {
    const createLandmark: CreateLandmark = {
      name,
      notes,
      biomes: selectedBiomes,
      dimension,
      tags,
      farms,
      coordinate: {
        x: parseInt(x),
        y: parseInt(y),
        z: parseInt(z),
      },
    };

    addLandmark.mutate(createLandmark);
  };

  if (
    isLoading ||
    isBiomesLoading ||
    isDimensionsLoading ||
    addLandmark.isLoading
  ) {
    return <Typography>Loading ...</Typography>;
  }

  const validLandmark = !!name && !!dimension && !!x && !!y && !!z;

  if (addLandmark.isError) {
    console.error(addLandmark.error);
  }

  return (
    <>
      {!world && <Typography>Unknown world ID</Typography>}
      {world && (
        <Paper>
          <Stack spacing={2}>
            <Typography variant="h3">{world.name}</Typography>
            <Stack spacing={1}>
              <Typography variant="h5">New Landmark Details</Typography>
              <Stack
                spacing={2}
                alignItems="start"
                sx={{ paddingLeft: "0.5em" }}
              >
                <Box>
                  <TextField
                    label="Landmark Name"
                    onChange={(event) => setName(event.currentTarget.value)}
                    value={name}
                    error={!name}
                  />
                </Box>
                <Box>
                  <Stack direction="row" spacing={1}>
                    <NumberField label="X" required value={x} setValue={setX} />
                    <NumberField label="Y" required value={y} setValue={setY} />
                    <NumberField label="Z" required value={z} setValue={setZ} />
                  </Stack>
                </Box>
                <FormControl>
                  <InputLabel>Dimension</InputLabel>
                  <Select
                    error={!dimension}
                    label="Dimension"
                    value={dimension}
                    onChange={handleDimensionChange}
                    style={{ minWidth: "20em" }}
                  >
                    {dimensions?.map((dim) => (
                      <MenuItem key={dim} value={dim}>
                        <Typography style={{ textTransform: "capitalize" }}>
                          {dim}
                        </Typography>
                      </MenuItem>
                    ))}
                  </Select>
                </FormControl>
                <FormControl>
                  <InputLabel>Biomes</InputLabel>
                  <Select
                    label="Biomes"
                    multiple
                    value={selectedBiomes}
                    onChange={handleBiomeChange}
                    style={{ minWidth: "20em" }}
                    renderValue={(selected) => (
                      <Box sx={{ display: "flex", flexWrap: "wrap", gap: 0.5 }}>
                        {selected.map((value) => (
                          <Chip
                            key={value}
                            label={value.replaceAll("_", " ")}
                            style={{ textTransform: "capitalize" }}
                          />
                        ))}
                      </Box>
                    )}
                  >
                    {biomes?.map((b) => (
                      <MenuItem key={b} value={b}>
                        <Typography style={{ textTransform: "capitalize" }}>
                          {b.replaceAll("_", " ")}
                        </Typography>
                      </MenuItem>
                    ))}
                  </Select>
                </FormControl>
                <TextListEditor
                  addButtonText="+ Add Tag"
                  textFieldLabel="New Tag"
                  values={tags}
                  valueAdded={(value) =>
                    setTags((current) => [...current, value])
                  }
                  valueRemoved={(value) =>
                    setTags((current) => {
                      const index = current.findIndex((val) => val === value);
                      if (index >= 0) {
                        const copy = current.slice();
                        copy.splice(index, 1);
                        return copy;
                      } else {
                        return current;
                      }
                    })
                  }
                />
                <TextListEditor
                  addButtonText="+ Add Farm"
                  textFieldLabel="New Farm"
                  values={farms}
                  valueAdded={(value) =>
                    setFarms((current) => [...current, value])
                  }
                  valueRemoved={(value) =>
                    setFarms((current) => {
                      const index = current.findIndex((val) => val === value);
                      if (index >= 0) {
                        const copy = current.slice();
                        copy.splice(index, 1);
                        return copy;
                      } else {
                        return current;
                      }
                    })
                  }
                />
                <Box>
                  <TextField
                    label="Notes"
                    onChange={(event) => setNotes(event.currentTarget.value)}
                    value={notes}
                  />
                </Box>
              </Stack>
              <Button
                variant="contained"
                disabled={!validLandmark}
                onClick={handleSubmitLandmark}
              >
                Add Landmark
              </Button>
            </Stack>
          </Stack>
        </Paper>
      )}
    </>
  );
};
