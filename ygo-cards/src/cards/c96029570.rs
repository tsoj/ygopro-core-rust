//! Ebon Sun
use super::prelude::*;

pub const ID: u32 = 96029570;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Gain LP equal to the total original ATK of the monsters destroyed
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1a, CATEGORY_RECOVER);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1a, EVENT_CUSTOM + ID);
    // e1a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1a, LOCATION_SZONE);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.lptg)
    effect::set_target(d, e1a, Some(cost_target(lptg)));
    // e1a:SetOperation(s.lpop)
    effect::set_operation(d, e1a, Some(operation(lpop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local g1=Group.CreateGroup()
    // g1:KeepAlive()
    // e1a:SetLabelObject(g1)
    effect::set_label_object(d, e1a, Value::Group(CardSet::new()));
    // Keep track of the destroyed monsters
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1b:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1b, EVENT_DESTROYED);
    // e1b:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1b, LOCATION_SZONE);
    // e1b:SetLabelObject(e1a)
    effect::set_label_object(d, e1b, Value::Effect(e1a));
    // e1b:SetOperation(s.atkregop)
    effect::set_operation(d, e1b, Some(operation(atkregop)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Increase the ATK of 1 monster by 1000
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2a, CATEGORY_ATKCHANGE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_CUSTOM+id+1)
    effect::set_code(d, e2a, EVENT_CUSTOM + ID + 1);
    // e2a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2a, LOCATION_SZONE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetTarget(s.atktg)
    effect::set_target(d, e2a, Some(cost_target(atktg)));
    // e2a:SetOperation(s.atkop)
    effect::set_operation(d, e2a, Some(operation(atkop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local g2=Group.CreateGroup()
    // g2:KeepAlive()
    // e2a:SetLabelObject(g2)
    effect::set_label_object(d, e2a, Value::Group(CardSet::new()));
    // Register monsters Special Summoned from your GY
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // e2b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2b:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2b, LOCATION_SZONE);
    // e2b:SetLabelObject(e2a)
    effect::set_label_object(d, e2b, Value::Effect(e2a));
    // e2b:SetOperation(s.regsumop)
    effect::set_operation(d, e2b, Some(operation(regsumop)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Add 1 discarded Spell/Trap to your hand
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3a, aux::stringid(ID, 2));
    // e3a:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3a, CATEGORY_TOHAND);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3a, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3a:SetCode(EVENT_CUSTOM+id+2)
    effect::set_code(d, e3a, EVENT_CUSTOM + ID + 2);
    // e3a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3a, LOCATION_SZONE);
    // e3a:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3a, 1, ID, 2, 0);
    // e3a:SetTarget(s.thtg)
    effect::set_target(d, e3a, Some(cost_target(thtg)));
    // e3a:SetOperation(s.thop)
    effect::set_operation(d, e3a, Some(operation(thop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local g3=Group.CreateGroup()
    // g3:KeepAlive()
    // e3a:SetLabelObject(g3)
    effect::set_label_object(d, e3a, Value::Group(CardSet::new()));
    // Register discarded Spell/Traps
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3b:SetCode(EVENT_DISCARD)
    effect::set_code(d, e3b, EVENT_DISCARD);
    // e3b:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3b, LOCATION_SZONE);
    // e3b:SetLabelObject(e3a)
    effect::set_label_object(d, e3b, Value::Effect(e3a));
    // e3b:SetOperation(s.stregop)
    effect::set_operation(d, e3b, Some(operation(stregop)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `e:GetLabelObject()` as a group (`Lua` label objects that hold a group).
fn label_group(d: &Duel, e: EffectId) -> CardSet {
    effect::get_label_object(d, e).as_group().cloned().nil()
}
/// `e:GetLabelObject()` as an effect.
fn label_effect(d: &Duel, e: EffectId) -> EffectId {
    effect::get_label_object(d, e).as_effect().nil()
}
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local value=e:GetLabelObject():GetSum(Card.GetBaseAttack)
    let g = label_group(d, a.e);
    let value = group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64);
    // if chk==0 then return value>0 end
    if a.chk == Some(0) {
        return value > 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(value)
    duel::set_target_param(d, value as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,value)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, value as i32);
    true
}
fn lpop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    let _ = a;
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
fn atkregop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.desfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| desfilter(d, c, a.tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e1a = label_effect(d, a.e);
        let mut g = label_group(d, e1a);
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e1a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let c = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
fn atkfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsSummonLocation(LOCATION_GRAVE) and c:IsPreviousControler(tp)
    card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_summon_location(d, c, LOCATION_GRAVE)
        && card::is_previous_controler(d, c, tp)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.atkfilter,nil,e,tp)
    let g0 = label_group(d, a.e);
    let g = group::filter(d, &g0, &|d, c| atkfilter(d, c, a.e, a.tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and chkc:IsLocation(LOCATION_MZONE) and s.atkfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && card::is_location(d, chkc, LOCATION_MZONE) && atkfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tc=nil
    let tc: Option<CardId>;
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        tc = group::get_first(d, &sel);
    } else {
        // tc=g:GetFirst()
        tc = group::get_first(d, &g);
    }
    // Duel.SetTargetCard(tc)
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, tc.map(CardSet::from_card), 1, a.tp, 1000);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
fn regsumop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.atkfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| atkfilter(d, c, a.e, a.tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id+1,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID + 1, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2a = label_effect(d, a.e);
        let mut g = label_group(d, e2a);
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id+1)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID + 1) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id+1,e,0,tp,tp,0)
        let c = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID + 1, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_GRAVE)
    //     and c:IsAbleToHand() and c:IsControler(tp) and c:IsPreviousControler(tp)
    card::is_spell_trap(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_able_to_hand(d, c, None)
        && card::is_controler(d, c, tp)
        && card::is_previous_controler(d, c, tp)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.thfilter,nil,e,tp)
    let g0 = label_group(d, a.e);
    let g = group::filter(d, &g0, &|d, c| thfilter(d, c, a.e, a.tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.cfilter(thfilter,e,tp) end
    // (s.cfilter is never defined in this script, so a contained chkc raises a Lua error)
    if let Some(chkc) = a.chkc() {
        if group::is_contains(&g, chkc) {
            crate::lua_error("attempt to call a nil value (field 'cfilter')");
        }
        return false;
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tc=nil
    let tc: Option<CardId>;
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        tc = group::get_first(d, &sel);
    } else {
        // tc=g:GetFirst()
        tc = group::get_first(d, &g);
    }
    // Duel.SetTargetCard(tc)
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, tc.map(CardSet::from_card), 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
fn stregop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.thfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| thfilter(d, c, a.e, a.tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id+2,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID + 2, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3a = label_effect(d, a.e);
        let mut g = label_group(d, e3a);
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id+2)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID + 2) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id+2,e,0,tp,tp,0)
        let c = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID + 2, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
