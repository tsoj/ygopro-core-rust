//! Angelechy Bastion
use super::prelude::*;

pub const ID: u32 = 28904860;
const CARD_ANGELECHY_SHATRANGA: u32 = 42410161;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ANGELECHY_SHATRANGA], listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuners
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // You can target 1 other card in this card's column; banish it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is placed in the Spell & Trap Zone as a Continuous Spell: You can place 1 "Angelechy Shatranga" from your Extra Deck in your Spell & Trap Zone as a face-up Continuous Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_STZONE)
    effect::set_range(d, e2, LOCATION_STZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.plcon)
    effect::set_condition(d, e2, Some(condition(plcon)));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While this card is treated as a Continuous Spell, other "Angelechy" cards on the field cannot be destroyed by your opponent's card effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetRange(LOCATION_STZONE)
    effect::set_range(d, e3, LOCATION_STZONE);
    // e3:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e3, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // e3:SetTarget(function(e,c) return c~=e:GetHandler() and c:IsSetCard(SET_ANGELECHY) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, e, c| {
        let h = effect::get_handler(d, e).nil();
        c != h && card::is_set_card(d, c, &[SET_ANGELECHY as u16])
    })));
    // e3:SetValue(aux.indoval)
    effect::set_value_fn(d, e3, aux::indoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local colg=c:GetColumnGroup():Match(Card.IsAbleToRemove,nil)
    let mut colg = card::get_column_group(d, c, 0, 0);
    group::match_(d, &mut colg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX);
    // if chkc then return colg:IsContains(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&colg, chkc) && chkc != c;
    }
    // if chk==0 then return colg:IsExists(Card.IsCanBeEffectTarget,1,nil,e) end
    if a.chk == Some(0) {
        return group::is_exists(d, &colg, &|d, c| card::is_can_be_effect_target(d, c, Some(e)), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=colg:FilterSelect(tp,Card.IsCanBeEffectTarget,1,1,nil,e)
    let g = group::filter_select(d, &colg, tp, &|d, c| card::is_can_be_effect_target(d, c, Some(e)), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    true
}
fn banop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
fn plcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return eg:IsContains(c) and not c:IsPreviousLocation(LOCATION_SZONE)
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, c) && !card::is_previous_location(d, c, LOCATION_SZONE)
}
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsCode(CARD_ANGELECHY_SHATRANGA) and not c:IsForbidden()
    card::is_code(d, c, &[CARD_ANGELECHY_SHATRANGA]) && !card::is_forbidden(d, c)
}
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return e:GetHandler():IsContinuousSpell()
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_continuous_spell(d, h)
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    true
}
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if let Some(sc) = sc {
        if duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
}
