//! Spirit Illusion
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 71939275;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID + 1], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip only to a Level 5 or higher Fiend or Zombie monster
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| eqfilter(d, c))), None, None, None, None, None, None);
    // Make the opponent's monster lose ATK equal to the equipped monster's
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(efftg)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
}
// function s.eqfilter(c)
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsRace(RACE_FIEND|RACE_ZOMBIE)
    card::is_level_above(d, c, 5) && card::is_race(d, c, RACE_FIEND | RACE_ZOMBIE)
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at,bt=Duel.GetBattleMonster(tp)
    let (at, bt) = duel::get_battle_monster(d, a.tp);
    // local ec=e:GetHandler():GetEquipTarget()
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c);
    // return at and bt and ec and at==ec and ec:GetAttack()>0 and bt:IsControler(1-tp)
    let (Some(at), Some(bt), Some(ec)) = (at, bt, ec) else { return false };
    at == ec && card::get_attack(d, ec) > 0 && card::is_controler(d, bt, 1 - a.tp)
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local at,bt=Duel.GetBattleMonster(tp)
    let (at, bt) = duel::get_battle_monster(d, a.tp);
    if a.chk == Some(0) {
        // return at:IsRelateToBattle() and bt:IsRelateToBattle()
        return card::is_relate_to_battle(d, at.nil()) && card::is_relate_to_battle(d, bt.nil());
    }
    let bt = bt.nil();
    // Duel.SetTargetCard(bt)
    duel::set_target_card(d, bt);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,bt,1,tp,-e:GetHandler():GetEquipTarget():GetAttack())
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c).nil();
    let atk = card::get_attack(d, ec);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(bt)), 1, a.tp, -atk);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // local bc=Duel.GetFirstTarget()
    let bc = duel::first_target(d);
    // if ec and bc:IsRelateToEffect(e) and bc:IsFaceup() and bc:IsControler(1-tp) then
    let Some(ec) = ec else { return };
    let bc = bc.nil();
    if !(card::is_relate_to_effect(d, bc, a.e) && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - a.tp)) {
        return;
    }
    // local atk=ec:GetAttack()
    let atk = card::get_attack(d, ec);
    // if atk<=0 then return end
    if atk <= 0 {
        return;
    }
    // Make that opponent's monster lose ATK equal to the equipped monster's
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(-atk)
    effect::set_value(d, e1, -(atk as i64));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
}
// The monster data of the Doppelganger Token for the equipped monster `ec`
fn token_data(d: &mut Duel, ec: CardId) -> duel::MonsterData {
    let atk = card::get_attack(d, ec);
    let race = card::get_race(d, ec);
    let attribute = card::get_attribute(d, ec);
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(atk),
        defense: Some(0),
        level: Some(5),
        race: Some(race),
        attribute: Some(attribute),
    }
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c);
    // local b1=Fusion.SummonEffTG()(e,tp,eg,ep,ev,re,r,rp,0)
    let mut a0 = a.clone();
    a0.chk = Some(0);
    let b1 = fusion::summon_eff_tg_call(d, &fusion::SummonEff::default(), &a0);
    // local b2=ec and Duel.GetLocationCount(tp,LOCATION_MZONE)
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,ec:GetAttack(),0,5,ec:GetRace(),ec:GetAttribute())
    let b2 = match ec {
        Some(ec) => {
            duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
            let md = token_data(d, ec);
            duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, md, None, None, 0)
        }
        None => false,
    };
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    let op = op.map_or(0, |x| x as i64);
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // Fusion.SummonEffTG()(e,tp,eg,ep,ev,re,r,rp,1)
        let mut a1 = a.clone();
        a1.chk = Some(1);
        fusion::summon_eff_tg_call(d, &fusion::SummonEff::default(), &a1);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
        // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Fusion.SummonEffOP()(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fusion::SummonEff::default(), a);
    } else if op == 2 {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // local ec=e:GetHandler():GetEquipTarget()
        let c = effect::get_handler(d, a.e).nil();
        let ec = card::get_equip_target(d, c).nil();
        // if not (ft>0 and Duel.IsPlayerCanSpecialSummonMonster(...)) then return end
        let md = token_data(d, ec);
        if !(ft > 0 && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, md, None, None, 0)) {
            return;
        }
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // token:Race(ec:GetRace())
        let race = card::get_race(d, ec);
        card::info_race(d, token, Some(race));
        // token:Attribute(ec:GetAttribute())
        let attribute = card::get_attribute(d, ec);
        card::info_attribute(d, token, Some(attribute));
        // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_ATTACK);
            // e1:SetValue(ec:GetAttack())
            let atk = card::get_attack(d, ec);
            effect::set_value(d, e1, atk as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1)
            card::register_effect(d, token, e1, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
