<script setup lang="ts">
import type { Todo } from "~/types";

// One item. Its only prop is the todo object, and the parent replaces that
// object when (and only when) this todo changes, so Vue's prop check skips
// re-rendering every other item.
const props = defineProps<{ todo: Todo }>();
const emit = defineEmits<{ toggle: [id: number, done: boolean]; destroy: [id: number] }>();
</script>

<template>
  <li :class="{ completed: props.todo.done }">
    <input
      class="toggle"
      type="checkbox"
      :checked="props.todo.done"
      @change="emit('toggle', props.todo.id, !props.todo.done)"
    >
    <label>{{ props.todo.title }}</label>
    <button class="destroy" @click="emit('destroy', props.todo.id)" />
  </li>
</template>
