//! Branded in Central Dogmatika
use super::prelude::*;

pub const ID: u32 = 14220547;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Look at either Extra Deck and send 1 monster from it to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.condition(TYPE_RITUAL))
    effect::set_condition(d, e2, Some(condition(|d, a| cond(d, a, TYPE_RITUAL))));
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Increase the ATK of your Fusion Monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.condition(TYPE_FUSION))
    effect::set_condition(d, e3, Some(condition(|d, a| cond(d, a, TYPE_FUSION))));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.condition(typ)(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs, typ: u32) -> bool {
    // if #eg~=1 then return false end
    let eg = a.eg.clone().nil();
    if eg.len() != 1 {
        return false;
    }
    // local c=eg:GetFirst()
    let c = group::get_first(d, &eg).nil();
    // return c:IsFaceup() and c:IsType(typ) and c:IsSummonPlayer(tp) and re and re:IsSpellEffect()
    if !card::is_faceup(d, c) || !card::is_type(d, c, typ) || !card::is_summon_player(d, c, a.tp) {
        return false;
    }
    match a.re {
        Some(re) => effect::is_spell_effect(d, re),
        None => false,
    }
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,LOCATION_EXTRA)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_EXTRA, LOCATION_EXTRA) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, LOCATION_EXTRA as i32);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetFieldGroup(tp,LOCATION_EXTRA,0)
    let g1 = duel::get_field_group(d, tp, LOCATION_EXTRA, 0);
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g2 = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
    // if not e:GetHandler():IsRelateToEffect(e) or #g1+#g2==0 then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) || g1.len() + g2.len() == 0 {
        return;
    }
    // local op=Duel.SelectEffect(tp,{#g1>0,aux.Stringid(id,2)},{#g2>0,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(!g1.is_empty(), aux::stringid(ID, 2)), (!g2.is_empty(), aux::stringid(ID, 3))]);
    // local g=(op==1) and g1 or g2
    let g = if op == Some(1) { g1.clone() } else { g2.clone() };
    // if op==2 then Duel.ConfirmCards(tp,g) end
    if op == Some(2) {
        duel::confirm_cards(d, tp, g.clone());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tg=g:FilterSelect(tp,aux.AND(Card.IsMonster,Card.IsAbleToGrave),1,1,nil)
    let tg = group::filter_select(d, &g, tp, &|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None), 1, 1, false, NO_EX).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoGrave(tg,REASON_EFFECT)
        duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
        // if op==2 then Duel.ShuffleExtra(1-tp) end
        if op == Some(2) {
            duel::shuffle_extra(d, 1 - tp);
        }
    }
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:GetFirst():IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let first = group::get_first(d, &eg).nil();
        return card::is_can_be_effect_target(d, first, Some(a.e));
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(tc:GetBaseAttack())
        let base = card::get_base_attack(d, tc) as i64;
        effect::set_value(d, e1, base);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
        effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // e2:SetValue(s.atlimit)
        effect::set_value_fn(d, e2, effect_card_fn(atlimit));
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e3, EFFECT_CANNOT_DIRECT_ATTACK);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return not (c:IsAttackPos() and c:IsControler(1-e:GetHandler():GetControler()))
    let h = effect::get_handler(d, e).nil();
    let p = card::get_controler(d, h);
    !(card::is_attack_pos(d, c) && card::is_controler(d, c, 1 - p))
}
