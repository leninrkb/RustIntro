<template>
    <Column>
        <div>
            <!-- header -->
            <Header></Header>
        </div>
        <div>
            <!-- search -->
            <Search></Search>
        </div>
        <div>
            <!-- options -->
            <ScrapingOptions></ScrapingOptions>
        </div>
        <div>
            <!-- content -->
            <Column>
                <div>
                    <!-- web page view -->
                </div>
                <div>
                    <!-- html view -->
                </div>
            </Column>
        </div>
    </Column>
</template>

<script setup>
import ScrapingOptions from '../components/ScrapingOptions.vue';
import Search from '../components/Search.vue';
import Header from '../components/Header.vue';
import Column from '../components/layout/Column.vue';
</script>