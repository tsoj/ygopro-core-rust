//! The Iris Swordsoul
use super::prelude::*;

pub const ID: u32 = 62849088;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_SUMMON|TIMING_SPSUMMON)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_SUMMON | TIMING_SPSUMMON));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetLabel(1)
    effect::set_label(d, e2, &[1]);
    // e2:SetCondition(s.hdexcon)
    effect::set_condition(d, e2, Some(condition(hdexcon)));
    // e2:SetTarget(s.hdextg)
    effect::set_target(d, e2, Some(cost_target(hdextg)));
    // e2:SetOperation(s.hdexop)
    effect::set_operation(d, e2, Some(operation(hdexop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Draw 2 cards
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetLabel(2)
    effect::set_label(d, e3, &[2]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy 1 of those monsters Special Summoned from the Extra Deck
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    // e4:SetLabel(3)
    effect::set_label(d, e4, &[3]);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e4:SetLabelObject(g)
    effect::set_label_object(d, e4, Value::Group(CardSet::new()));
    // Register summons
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetLabelObject(g)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // e5:SetOperation(s.regop)
    effect::set_operation(d, e5, Some(operation(regop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// The effect that holds the shared group (e4): itself, or via `Value::Effect` for e5.
fn group_owner(d: &Duel, e: EffectId) -> EffectId {
    match effect::get_label_object(d, e) {
        Value::Effect(e4) => e4,
        _ => e,
    }
}
/// The group a card script keeps in an effect's label object (`e:GetLabelObject()`, a group).
fn label_group(d: &Duel, e: EffectId) -> CardSet {
    match effect::get_label_object(d, group_owner(d, e)) {
        Value::Group(g) => g,
        _ => crate::lua_error("attempt to index a nil value"),
    }
}
/// `s.spcfilter(c)`
fn spcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsDisabled() and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_disabled(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and Duel.IsExistingMatchingCard(s.spcfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_main_phase(d, None) &&duel::is_existing_matching_card(d, Some(&spcfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.hdexfilter(c,tp,loc)`
fn hdexfilter(d: &mut Duel, c: CardId, tp: u8, loc: u32) -> bool {
    // return c:IsOriginalType(TYPE_MONSTER) and c:IsSummonPlayer(1-tp) and c:IsSummonLocation(loc)
    //     and (loc~=LOCATION_EXTRA or c:IsOnField())
    card::is_original_type(d, c, TYPE_MONSTER)
        && card::is_summon_player(d, c, 1 - tp)
        && card::is_summon_location(d, c, loc)
        && (loc != LOCATION_EXTRA || card::is_on_field(d, c))
}
/// `s.hdexcon(e,tp,eg,ep,ev,re,r,rp)`
fn hdexcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // if c:GetFlagEffect(id+label)>0 then return false end
    if card::get_flag_effect(d, c, ID + label as u32) > 0 {
        return false;
    }
    // local loc=0
    let mut loc = 0;
    // local eg as the event group (or the label group for label 3)
    let mut eg = a.eg.clone();
    // if label==1 then loc=LOCATION_HAND
    if label == 1 {
        loc = LOCATION_HAND;
    } else if label == 2 {
        // elseif label==2 then loc=LOCATION_DECK
        loc = LOCATION_DECK;
    } else if label == 3 {
        // elseif label==3 then loc=LOCATION_EXTRA; eg=e:GetLabelObject()
        loc = LOCATION_EXTRA;
        eg = Some(label_group(d, a.e));
    }
    // if eg and eg:IsExists(s.hdexfilter,1,nil,tp,loc) then
    if let Some(eg) = eg {
        if group::is_exists(d, &eg, &|d, c| hdexfilter(d, c, tp, loc), 1, NO_EX) {
            // if Duel.GetCurrentChain()>0 then
            if duel::get_current_chain(d, false) > 0 {
                // c:RegisterFlagEffect(id+label,RESET_CHAIN,0,1)
                card::register_flag_effect(d, c, ID + label as u32, RESET_CHAIN, 0, 1, 0, 0);
            }
            // return true
            return true;
        }
    }
    // return false
    false
}
/// `s.hdextg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdextg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local label=e:GetLabel()
    let label = effect::get_label(d, e);
    // if chk==0 then
    if a.chk == Some(0) {
        // if label==1 then
        if label == 1 {
            // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
            //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
            return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
        } else if label == 2 {
            // elseif label==2 then return Duel.IsPlayerCanDraw(tp,2)
            return duel::is_player_can_draw(d, tp, 2);
        } else if label == 3 {
            // local g=e:GetLabelObject():Filter(s.hdexfilter,nil,tp,LOCATION_EXTRA)
            // return #g>0
            let lg = label_group(d, e);
            let g = group::filter(d, &lg, &|d, c| hdexfilter(d, c, tp, LOCATION_EXTRA), NO_EX);
            return !g.is_empty();
        }
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, e));
    // if label==1 then
    if label == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    } else if label == 2 {
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, tp);
        // Duel.SetTargetParam(2)
        duel::set_target_param(d, 2);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    } else if label == 3 {
        // local g=e:GetLabelObject():Filter(s.hdexfilter,nil,tp,LOCATION_EXTRA)
        let lg = label_group(d, e);
        let g = group::filter(d, &lg, &|d, c| hdexfilter(d, c, tp, LOCATION_EXTRA), NO_EX);
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g.clone());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    true
}
/// `s.hdexop(e,tp,eg,ep,ev,re,r,rp)`
fn hdexop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local label=e:GetLabel()
    let label = effect::get_label(d, e);
    // if label==1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if label == 1 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    } else if label == 2 {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        let p = info[0].int() as u8;
        let n = info[1].int() as u32;
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, p, n, REASON_EFFECT);
    } else if label == 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=e:GetLabelObject():Match(Card.IsRelateToEffect,nil,e)
        let lg = label_group(d, e);
        let g = group::filter(d, &lg, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
        // g=g:FilterSelect(tp,s.hdexfilter,1,1,nil,tp,LOCATION_EXTRA)
        let g = group::filter_select(d, &g, tp, &|d, c| hdexfilter(d, c, tp, LOCATION_EXTRA), 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id)>0 then return end
    if duel::get_flag_effect(d, tp, ID) > 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=eg:Filter(s.hdexfilter,nil,tp,LOCATION_EXTRA)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, cc| hdexfilter(d, cc, tp, LOCATION_EXTRA), NO_EX);
    // if #tg>0 and not tg:IsContains(c) then
    if !tg.is_empty() && !group::is_contains(&tg, c) {
        // for tc in aux.Next(tg) do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject()
        let owner = group_owner(d, a.e);
        let mut g = label_group(d, a.e);
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, cc| card::get_flag_effect(d, cc, ID) == 0, NO_EX);
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        effect::set_label_object(d, owner, Value::Group(g));
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
    }
}
