<script lang="ts">
  import { calc } from './calc';
  $: snapshot = calc.snapshot;
</script>

<div class="display">
  {#if $snapshot.error}
    <div class="error">{$snapshot.error}</div>
  {/if}
  <div class="value">{$snapshot.display}</div>
</div>

<style>
  .display {
    background: #0e1623;
    color: #f4f6fa;
    padding: 1.2rem 1rem;
    border-radius: 0.75rem;
    text-align: right;
    font-family: 'JetBrains Mono', ui-monospace, monospace;
    border: 1px solid rgba(255, 255, 255, 0.06);
    min-height: 4.5rem;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
  }
  .value {
    font-size: 2rem;
    font-weight: 600;
    word-break: break-all;
  }
  .error {
    color: #ff7676;
    font-size: 0.8rem;
    text-align: left;
    margin-bottom: 0.3rem;
  }
</style>
