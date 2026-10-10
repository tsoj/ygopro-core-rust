//! Cupid Pitch
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 21915012;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Check the Tuner material's Level
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetLabel(0)
    effect::set_label(d, e0, &[0]);
    // e0:SetValue(s.valcheck)
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    // e0:SetOperation(s.matop)
    effect::set_operation(d, e0, Some(lua_fn(matop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Increase or decrease this card's Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_synchro_summoned(d, h)
    })));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains ATK equal to its Level x 400
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(_,c) return c:GetLevel()*400 end)
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, c| card::get_level(d, c) as i64 * 400));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict damage and search 1 Level 8 or lower monster with 600 DEF
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DAMAGE+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_DAMAGE + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.puretuner(c,sc,tp)`
fn puretuner(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return not c:IsHasEffect(EFFECT_NONTUNER) and c:IsType(TYPE_TUNER,sc,SUMMON_TYPE_SYNCHRO|MATERIAL_SYNCHRO,tp)
    card::is_has_effect(d, c, EFFECT_NONTUNER, None).is_empty()
        && card::is_type_ctx(d, c, TYPE_TUNER, Some(sc), (SUMMON_TYPE_SYNCHRO as u64) | MATERIAL_SYNCHRO, Some(tp))
}
/// `s.othertuner(c,sc,tp)`
fn othertuner(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return (c:IsHasEffect(EFFECT_NONTUNER) and c:IsType(TYPE_TUNER,sc,SUMMON_TYPE_SYNCHRO|MATERIAL_SYNCHRO,tp)) or c:IsHasEffect(EFFECT_CAN_BE_TUNER)
    (!card::is_has_effect(d, c, EFFECT_NONTUNER, None).is_empty()
        && card::is_type_ctx(d, c, TYPE_TUNER, Some(sc), (SUMMON_TYPE_SYNCHRO as u64) | MATERIAL_SYNCHRO, Some(tp)))
        || !card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty()
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // if #mg==0 then return e:SetLabel(0) end
    if mg.is_empty() {
        effect::set_label(d, e, &[0]);
        return 0;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local pure_tuner_g=mg:Filter(s.puretuner,nil,c,tp)
    let pure_tuner_g = group::filter(d, &mg, &|d, x| puretuner(d, x, c, tp), NO_EX);
    // if #pure_tuner_g>0 then
    if !pure_tuner_g.is_empty() {
        // e:SetLabelObject(nil)
        effect::set_label_object(d, e, Value::Nil);
        // e:SetLabel(pure_tuner_g:GetFirst():GetSynchroLevel(c))
        let first = group::get_first(d, &pure_tuner_g).nil();
        let lv = card::get_synchro_level(d, first, c);
        effect::set_label(d, e, &[lv as i64]);
        return 0;
    }
    // local other_tuner_g=mg:Filter(s.othertuner,nil,c,tp)
    let other_tuner_g = group::filter(d, &mg, &|d, x| othertuner(d, x, c, tp), NO_EX);
    // if #other_tuner_g>1 then
    if other_tuner_g.len() > 1 {
        // e:SetLabelObject(other_tuner_g)
        effect::set_label_object(d, e, Value::Group(other_tuner_g));
    } else {
        // e:SetLabelObject(nil)
        effect::set_label_object(d, e, Value::Nil);
        // e:SetLabel(other_tuner_g:GetFirst():GetSynchroLevel(c))
        let first = group::get_first(d, &other_tuner_g).nil();
        let lv = card::get_synchro_level(d, first, c);
        effect::set_label(d, e, &[lv as i64]);
    }
    0
}
/// `s.matop(e,c)`
fn matop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local c=... (the card: the second argument of the core's call)
    let c = a.card(1).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,5))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 5));
    // local tuner=g:Select(tp,1,1,nil):GetFirst()
    let tuner_g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let tuner = group::get_first(d, &tuner_g).nil();
    // Duel.HintSelection(tuner,true)
    duel::hint_selection(d, CardSet::from_card(tuner), true);
    // e:SetLabel(tuner:GetSynchroLevel(c))
    let lv = card::get_synchro_level(d, tuner, c);
    effect::set_label(d, e, &[lv as i64]);
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local lv=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let lv = effect::get_label(d, lo);
    // if lv==0 then return end
    if lv == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local opt=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
        let opt = duel::select_option(d, tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // if opt==0 then e1:SetValue(lv) else e1:SetValue(-lv) end
        if opt == 0 {
            effect::set_value(d, e1, lv);
        } else {
            effect::set_value(d, e1, -lv);
        }
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsSynchroSummoned() and c:IsLocation(LOCATION_GRAVE) and r&REASON_SYNCHRO>0
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_synchro_summoned(d, c)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (a.r & REASON_SYNCHRO) > 0
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local sc=e:GetHandler():GetReasonCard()
    let h = effect::get_handler(d, a.e).nil();
    let sc = card::get_reason_card(d, h).nil();
    // Duel.SetTargetCard(sc)
    duel::set_target_card(d, CardSet::from_card(sc));
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,sc:GetLevel()*100)
    let lv = card::get_level(d, sc) as i32 * 100;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, lv);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(8) and c:IsDefense(600) and c:IsAbleToHand()
    card::is_level_below(d, c, 8) && card::is_defense(d, c, &[600]) && card::is_able_to_hand(d, c, None)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sc=Duel.GetFirstTarget()
    let sc = duel::first_target(d).nil();
    // if not (sc:IsRelateToEffect(e) and sc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, sc, a.e) && card::is_faceup(d, sc)) {
        return;
    }
    // if Duel.Damage(1-tp,sc:GetLevel()*100,REASON_EFFECT)>0
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
    let amount = card::get_level(d, sc) as i64 * 100;
    if duel::damage(d, 1 - tp, amount, REASON_EFFECT, false, None) > 0
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
