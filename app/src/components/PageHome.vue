<script setup>
import { ref, watchEffect } from "vue";
import { fetchTweets } from "@/api";
import TweetForm from "@/components/TweetForm";
import TweetList from "@/components/TweetList";
import { useWorkspace } from "@/composables";

const tweets = ref([]);
const loading = ref(true);
const { wallet } = useWorkspace();

watchEffect(() => {
  if (!wallet.value) return;
  fetchTweets()
    .then((fetchedTweets) => (tweets.value = fetchedTweets))
    .finally(() => (loading.value = false));
});

const addTweet = (tweet) => tweets.value.push(tweet);
</script>

<template>
  <tweet-form @added="addTweet"></tweet-form>
  <tweet-list :tweets="tweets" :loading="loading"></tweet-list>
</template>
