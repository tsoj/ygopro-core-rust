//! Cyberdark Edge
use super::prelude::*;

pub const ID: u32 = 77625948;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBERDARK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(Cyberdark.EquipTarget(s.eqfilter,true,true))
    effect::set_target(d, e1, Some(aux::cyberdark::equip_target(Some(std::sync::Arc::new(eqfilter)), true, true)));
    // e1:SetOperation(Cyberdark.EquipOperation(s.eqfilter,s.equipop,true))
    effect::set_operation(d, e1, Some(aux::cyberdark::equip_operation(Some(std::sync::Arc::new(eqfilter)), lua_fn(equipop), true)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop), e1, None, None, None);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk change
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(3) and c:IsRace(RACE_DRAGON)
    card::is_level_below(d, c, 3) && card::is_race(d, c, RACE_DRAGON)
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, a: Args) -> bool {
    // return ec:IsControler(tp) and ec:IsLevelBelow(3) and ec:IsRace(RACE_DRAGON)
    card::is_controler(d, a.c(0), a.p(2)) && card::is_level_below(d, a.c(0), 3) && card::is_race(d, a.c(0), RACE_DRAGON)
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, a: Args) {
    let (c, e, tp, tc) = (a.c(0), a.e(1), a.p(2), a.c(3));
    // local atk=tc:GetTextAttack()
    let mut atk = card::get_text_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_OWNER_RELATE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_OWNER_RELATE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // e2:SetValue(atk)
    effect::set_value(d, e2, atk as i64);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_DESTROY_SUBSTITUTE)
    effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    // e3:SetValue(s.repval)
    effect::set_value_fn(d, e3, lua_fn(repval));
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
}
/// `s.repval(e,re,r,rp)`
fn repval(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    ((a.int(2) as u32) & REASON_BATTLE) != 0
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()==nil and e:GetHandler():IsHasEffect(EFFECT_DIRECT_ATTACK)
    //   and Duel.IsExistingMatchingCard(aux.NOT(Card.IsHasEffect),tp,0,LOCATION_MZONE,1,nil,EFFECT_IGNORE_BATTLE_TARGET)
    if duel::get_attack_target(d).is_some() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::is_has_effect(d, h, EFFECT_DIRECT_ATTACK, None).is_empty() {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_has_effect(d, c, EFFECT_IGNORE_BATTLE_TARGET, None).is_empty()), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local effs={c:GetCardEffect(EFFECT_DIRECT_ATTACK)}
    let effs = card::get_card_effect(d, c, EFFECT_DIRECT_ATTACK);
    // local eg=Group.CreateGroup()
    let mut eg = CardSet::new();
    // for _,eff in ipairs(effs) do eg:AddCard(eff:GetOwner()) end
    for eff in effs {
        group::add_card(&mut eg, &CardSet::from_card(effect::get_owner(d, eff).nil()));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EFFECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EFFECT as u64);
    // local ec = #eg==1 and eg:GetFirst() or eg:Select(tp,1,1,nil):GetFirst()
    let first = if eg.len() == 1 { group::get_first(d, &eg) } else { None };
    let ec = match first {
        Some(tc) => Some(tc),
        None => {
            let sel = group::select(d, &eg, a.tp, 1, 1, false, NO_EX).nil();
            group::get_first(d, &sel)
        }
    };
    // if c==ec then
    if Some(c) == ec {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(c:GetAttack()/2)
        let atk = card::get_attack(d, c);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
