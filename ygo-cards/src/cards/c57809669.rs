//! Hecahands Tartaros
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 57809669;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HECAHANDS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects (but you can only use each effect of "Hecahands Tartarus" once per turn)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you use a face-up monster(s) you control owned by your opponent, you can treat it as a "Hecahands" monster
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local e0a=Effect.CreateEffect(c)
        let e0a = effect::create_effect(d, c);
        // e0a:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e0a, EFFECT_TYPE_FIELD);
        // e0a:SetCode(EFFECT_ADD_SETCODE)
        effect::set_code(d, e0a, EFFECT_ADD_SETCODE);
        // e0a:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e0a, LOCATION_MZONE, 0);
        // e0a:SetTarget(function(e,c) local tp=e:GetHandlerPlayer() return c:IsFaceup() and c:IsControler(tp) and c:IsOwner(1-tp) end)
        effect::set_target(
            d,
            e0a,
            Some(effect_card_fn(|d, e, c| {
                let tp = effect::get_handler_player(d, e);
                card::is_faceup(d, c) && card::is_controler(d, c, tp) && card::is_owner(d, c, 1 - tp)
            })),
        );
        // e0a:SetOperation(s.chngcon)
        effect::set_operation(d, e0a, Some(lua_fn(chngcon)));
        // e0a:SetValue(SET_HECAHANDS)
        effect::set_value(d, e0a, SET_HECAHANDS as i64);
        // Duel.RegisterEffect(e0a,0)
        duel::register_effect(d, e0a, 0);
        // local e0b=e0a:Clone()
        let e0b = effect::clone(d, e0a);
        // Duel.RegisterEffect(e0b,1)
        duel::register_effect(d, e0b, 1);
    });
}
/// `s.chngcon(scard,sumtype,tp)`
fn chngcon(d: &mut Duel, a: Args) -> bool {
    // return Fusion.SummonEffect and Fusion.SummonEffect:GetHandler():IsCode(id) and ((sumtype&MATERIAL_FUSION)~=0 or (sumtype&SUMMON_TYPE_FUSION)~=0)
    let sumtype = a.int(1) as u64;
    let Some(se) = fusion::summon_effect(d) else {
        return false;
    };
    let h = effect::get_handler(d, se).nil();
    card::is_code(d, h, &[ID]) && ((sumtype & MATERIAL_FUSION as u64) != 0 || (sumtype & SUMMON_TYPE_FUSION as u64) != 0)
}
/// `Fusion.SummonEffTG(fusion_params)` / `Fusion.SummonEffOP(fusion_params)`: the parameters of the
/// Fusion Summon in both effects (`fusfilter`, `matfilter`, `extrafil`, `extraop`, `extratg`)
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_HECAHANDS)
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_HECAHANDS as u16]))),
        // matfilter=Fusion.OnFieldMat(Card.IsAbleToRemove)
        matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, tp, chk| {
            card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32))
        }))),
        // extrafil=s.fextra
        extrafil: Some(Arc::new(fextra)),
        // extraop=Fusion.BanishMaterial
        extraop: Some(fusion::banish_material()),
        // extratg=s.extratg
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// `Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,chk)` with the given `chk`
fn summon_tg(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let mut args = a.clone();
    args.chk = Some(chk);
    fusion::summon_eff_tg_call(d, &fusion_params(), &args)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            LOCATION_GRAVE,
            0,
            NO_EX,
        );
        return (Some(g), None);
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_HECAHANDS),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,nil)
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HECAHANDS as u16])),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
        && duel::is_existing_target(d, Some(&|d, c| card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && summon_tg(d, a, 0);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return e:GetLabel()==1 and chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, e) == 1 && card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, e) != -100;
    // local b1=(cost_skip or not Duel.HasFlagEffect(tp,id))
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_HECAHANDS),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,nil)
    let b1 = (cost_skip || !duel::has_flag_effect(d, tp, ID, None))
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HECAHANDS as u16])),
            tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
        && duel::is_existing_target(d, Some(&|d, c| card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // local b2=(cost_skip or not Duel.HasFlagEffect(tp,id+1)) and Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = (cost_skip || !duel::has_flag_effect(d, tp, ID + 1, None)) && summon_tg(d, a, 0);
    // if chk==0 then e:SetLabel(0) return b1 or b2 end
    if a.chk == Some(0) {
        effect::set_label(d, e, &[0]);
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectTarget(tp,aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(
            d,
            tp,
            Some(&|d, c| card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)),
            tp,
            0,
            LOCATION_ONFIELD,
            1,
            1,
            false,
            NO_EX,
        );
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // e:SetProperty(0)
        effect::set_property(d, e, 0, 0);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,1)
        summon_tg(d, a, 1);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Return 1 Spell/Trap your opponent controls to the hand
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    } else if op == 2 {
        // Fusion Summon 1 "Hecahands" Fusion Monster from your Extra Deck, by banishing its materials from your field and/or GY
        // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fusion_params(), a);
    }
}
