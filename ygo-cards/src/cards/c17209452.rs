//! Kewl Tune Rotary
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 17209452;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KEWL_TUNE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card on the field would be used as Synchro Material, 1 Tuner in your hand can be used as 1 of the other materials
    // Synchro.AddHandMaterialEffect(c,id,function(c) return c:IsType(TYPE_TUNER) end)
    synchro::add_hand_material_effect(d, c, ID, Some(std::sync::Arc::new(|d: &mut Duel, c: CardId| card::is_type(d, c, TYPE_TUNER))), None, false, None);
    // You can reveal this card and 1 other Tuner in your hand; immediately after this effect resolves, Normal Summon 1 Tuner
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.nscost)
    effect::set_cost(d, e1, Some(cost_target(nscost)));
    // e1:SetTarget(s.nstg)
    effect::set_target(d, e1, Some(cost_target(nstg)));
    // e1:SetOperation(s.nsop)
    effect::set_operation(d, e1, Some(operation(nsop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is sent to the GY as Synchro Material: You can activate 1 of these effects;
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO end
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && a.r == REASON_SYNCHRO
}
// function s.nscostfilter(c)
fn nscostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TUNER) and not c:IsPublic()
    card::is_type(d, c, TYPE_TUNER) && !card::is_public(d, c)
}
// function s.nscost(e,tp,eg,ep,ev,re,r,rp,chk)
fn nscost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsPublic() and Duel.IsExistingMatchingCard(s.nscostfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return !card::is_public(d, c) && duel::is_existing_matching_card(d, Some(&nscostfilter), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.nscostfilter,tp,LOCATION_HAND,0,1,1,c):GetFirst()
    let rcg = duel::select_matching_card(d, tp, Some(&nscostfilter), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    let rc = group::get_first(d, &rcg).nil();
    // Duel.ConfirmCards(1-tp,Group.FromCards(c,rc))
    let mut both = CardSet::from_card(c);
    group::add_card(&mut both, &CardSet::from_card(rc));
    duel::confirm_cards(d, 1 - tp, both);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
// function s.nsfilter(c)
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsSummonable(true,nil)
    card::is_type(d, c, TYPE_TUNER) && card::is_summonable(d, c, true, None, 0, None)
}
// function s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
// function s.nsop(e,tp,eg,ep,ev,re,r,rp)
fn nsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let scg = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &scg) {
        // Duel.Summon(tp,sc,true,nil)
        duel::summon(d, tp, sc, true, None, 0, None);
    }
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    // local b2=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0
    let b2 = duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0;
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(choice));
    // if cd.choice==1 then
    if choice == 1 {
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,1-tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - tp, LOCATION_GRAVE as i32);
    // elseif cd.choice==2 then
    } else if choice == 2 {
        // e:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_SEARCH + CATEGORY_TOHAND);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_KEWL_TUNE) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_KEWL_TUNE as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cd=e:GetChainData()
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // if cd.choice==1 then
    if choice == 1 {
        // --● Place 1 card from your opponent's GY on the bottom of the Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,0,LOCATION_GRAVE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
    // elseif cd.choice==2 then
    } else if choice == 2 {
        // --● Look at your opponent's hand, then you can add 1 "Kewl Tune" Spell/Trap from your Deck to your hand
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
        let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, tp, g);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
        // local hg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let hg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if #hg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,4)) then return end
        if hg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local hsg=hg:Select(tp,1,1,nil)
        let hsg = group::select(d, &hg, tp, 1, 1, false, NO_EX).nil();
        // if #hsg>0 then
        if !hsg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(hsg,nil,REASON_EFFECT)
            duel::send_to_hand(d, hsg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,hsg)
            duel::confirm_cards(d, 1 - tp, hsg);
        }
    }
}
