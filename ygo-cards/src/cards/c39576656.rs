//! Kewl Tune Crackle
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 39576656;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[43904702], material: &[43904702], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Kewl Tune Clip" + 1+ Tuners
    // Synchro.AddProcedure(c,aux.FALSE,1,1,s.tunerfilter,1,99,aux.FilterSummonCode(43904702))
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|_d, _c, _sc, _st, _tp| false)),
        1,
        1,
        Some(mat_filter(tunerfilter)),
        1,
        99,
        Some(aux::filter_summon_code(&[43904702])),
        None,
        None,
        None,
        None,
    );
    // If this card is Synchro Summoned: You can look at your opponent's Extra Deck, and if you do, banish (face-up) 1 card from it (until the End Phase), then you can make this card gain ATK equal to the banished monster's
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this Synchro Summoned card is sent to the GY: You can Special Summon this card, then you can look at your opponent's Extra Deck, and if you do, banish (face-up) 2 cards from it until the End Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) local c=e:GetHandler() return c:IsSynchroSummoned() and c:IsPreviousLocation(LOCATION_MZONE) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Multiple tuners
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.tunerfilter(c,scard,sumtype,tp)
fn tunerfilter(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER,scard,sumtype,tp) or c:IsHasEffect(EFFECT_CAN_BE_TUNER)
    card::is_type_ctx(d, c, TYPE_TUNER, scard, sumtype, Some(tp)) || !card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty()
}
// function(e) return e:GetHandler():IsSynchroSummoned() end
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
// function(e) local c=e:GetHandler() return c:IsSynchroSummoned() and c:IsPreviousLocation(LOCATION_MZONE) end
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c) && card::is_previous_location(d, c, LOCATION_MZONE)
}
// function s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)>0 and Duel.IsPlayerCanRemove(tp) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_EXTRA) > 0 && duel::is_player_can_remove(d, tp, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
// function s.banop(e,tp,eg,ep,ev,re,r,rp)
fn banop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rc=g:FilterSelect(tp,Card.IsAbleToRemove,1,1,nil):GetFirst()
    let rcg = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &rcg);
    // if rc and aux.RemoveUntil(rc,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY,PHASE_END,id,e,tp,s.retop,nil,nil,nil,nil,aux.Stringid(id,2))
    //     and rc:IsLocation(LOCATION_REMOVED) and rc:IsFaceup() then
    if let Some(rc) = rc {
        if aux::remove_until(
            d,
            rc,
            Some(POS_FACEUP as u8),
            REASON_EFFECT,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(retop_op()),
            None,
            None,
            None,
            None,
            Some(aux::stringid(ID, 2)),
        )
        .is_some()
            && card::is_location(d, rc, LOCATION_REMOVED)
            && card::is_faceup(d, rc)
        {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local atk=rc:GetAttack()
            let atk = card::get_attack(d, rc);
            // if c:IsRelateToEffect(e) and c:IsFaceup() and atk>0
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
            if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && atk > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // c:UpdateAttack(atk)
                card::update_attack(d, c, atk as i64, None, None, None);
            }
        }
    }
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - tp);
}
// s.retop used as the operation of aux.RemoveUntil
fn retop_op() -> aux::DelayedFn {
    std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
        retop(d, ag);
        true
    })
}
// function s.retop(ag)
fn retop(d: &mut Duel, ag: &CardSet) {
    // local facedown_g,faceup_g=ag:Split(Card.IsPreviousPosition,nil,POS_FACEDOWN)
    let (facedown_g, faceup_g) = group::split(d, ag, &|d, c| card::is_previous_position(d, c, POS_FACEDOWN as u32), NO_EX);
    // if #facedown_g>0 then
    if !facedown_g.is_empty() {
        // Duel.SendtoDeck(facedown_g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, facedown_g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // Duel.ShuffleExtra(facedown_g:GetFirst():GetControler())
        let first = group::get_first(d, &facedown_g).nil();
        let p = card::get_controler(d, first);
        duel::shuffle_extra(d, p);
    }
    // if #faceup_g>0 then
    if !faceup_g.is_empty() {
        // Duel.SendtoExtraP(faceup_g,nil,REASON_EFFECT)
        duel::send_to_extra_p(d, faceup_g, None, REASON_EFFECT, None);
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,2,1-tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 2, 1 - tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
    // if #g<2 or not Duel.IsPlayerCanRemove(tp) or not Duel.SelectYesNo(tp,aux.Stringid(id,4)) then return end
    if g.len() < 2 || !duel::is_player_can_remove(d, tp, None) || !duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=g:FilterSelect(tp,Card.IsAbleToRemove,2,2,nil)
    let rg = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_remove(d, c, None, None, None), 2, 2, false, NO_EX).nil();
    // if #rg==2 then
    if rg.len() == 2 {
        // aux.RemoveUntil(rg,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY,PHASE_END,id,e,tp,s.retop,nil,nil,nil,nil,aux.Stringid(id,5))
        aux::remove_until(
            d,
            rg,
            Some(POS_FACEUP as u8),
            REASON_EFFECT,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(retop_op()),
            None,
            None,
            None,
            None,
            Some(aux::stringid(ID, 5)),
        );
    }
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - tp);
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(function(c) return c:IsType(TYPE_TUNER) or c:IsHasEffect(EFFECT_CAN_BE_TUNER) end,2,nil) then
    if group::is_exists(
        d,
        &g,
        &|d, x| card::is_type(d, x, TYPE_TUNER) || !card::is_has_effect(d, x, EFFECT_CAN_BE_TUNER, None).is_empty(),
        2,
        NO_EX,
    ) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetCode(EFFECT_MULTIPLE_TUNERS)
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD)|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    0
}
