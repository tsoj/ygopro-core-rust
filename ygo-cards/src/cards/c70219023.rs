//! Coordius the Triphasic Dealmon
use super::prelude::*;

pub const ID: u32 = 70219023;
pub const INFO: ScriptInfo = ScriptInfo { miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.fusfilter(TYPE_SYNCHRO),s.fusfilter(TYPE_XYZ),s.fusfilter(TYPE_LINK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_type_ctx(d, x, TYPE_SYNCHRO, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_type_ctx(d, x, TYPE_XYZ, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_type_ctx(d, x, TYPE_LINK, Some(fc), st, Some(tp)))),
        ],
    );
    // Apply up to 3 effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DESTROY+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DESTROY + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,0)`: the check of the search part
fn thtg_check(d: &mut Duel, tp: u8) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.destg(e,tp,...,0)`: the check of the destroy part
fn destg_check(d: &mut Duel, tp: u8) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>=3 end
    duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) >= 3
}
/// `s.atktg(e,tp,...,0)`: the check of the ATK part
fn atktg_check(d: &mut Duel, e: EffectId, tp: u8) -> bool {
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) and Duel.GetLP(tp)-Duel.GetLP(1-tp)~=0 end
    let h = effect::get_handler(d, e).nil();
    card::is_relate_to_effect(d, h, e) && duel::get_lp(d, tp) - duel::get_lp(d, 1 - tp) != 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=0
    let mut ct: i64 = 0;
    // if s.thtg(e,tp,eg,ep,ev,re,r,rp,0) then ct=ct+1 end
    if thtg_check(d, a.tp) {
        ct += 1;
    }
    // if s.destg(e,tp,eg,ep,ev,re,r,rp,0) then ct=ct+1 end
    if destg_check(d, a.tp) {
        ct += 1;
    }
    // if s.atktg(e,tp,eg,ep,ev,re,r,rp,0) then ct=ct+1 end
    if atktg_check(d, a.e, a.tp) {
        ct += 1;
    }
    // if chk==0 then return ct>0 and Duel.CheckLPCost(tp,2000) end
    if a.chk == Some(0) {
        return ct > 0 && duel::check_lp_cost(d, a.tp, 2000);
    }
    // ct=math.min(ct,Duel.GetLP(tp)//2000)
    let lp = duel::get_lp(d, a.tp) as i64;
    ct = ct.min(lp.div_euclid(2000));
    // local t={}
    // for i=1,ct do t[i]=i*2000 end
    let t: Vec<u64> = (1..=ct).map(|i| (i * 2000) as u64).collect();
    // local cost=Duel.AnnounceNumber(tp,table.unpack(t))
    let cost = duel::announce_number(d, a.tp, &t).nil().0 as i64;
    // Duel.PayLPCost(tp,cost)
    duel::pay_lp_cost(d, a.tp, cost as u32);
    // e:SetLabel(cost/2000)
    effect::set_label(d, a.e, &[cost / 2000]);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,3,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 3, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local opt=0
    let mut opt: i64 = 0;
    // local optp=0
    let mut optp: i64 = 0;
    // local b1=s.thtg(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = thtg_check(d, a.tp);
    // local b2=s.destg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = destg_check(d, a.tp);
    // local b3=s.atktg(e,tp,eg,ep,ev,re,r,rp,0)
    let b3 = atktg_check(d, a.e, a.tp);
    // for i=1,ct do
    for _ in 1..=ct {
        // local idtable={}
        let mut idtable: Vec<i64> = Vec::new();
        // local desctable={}
        let mut desctable: Vec<u64> = Vec::new();
        // if b1 and (opt&1)==0 then
        if b1 && (opt & 1) == 0 {
            // idtable[t]=1
            idtable.push(1);
            // desctable[t]=aux.Stringid(id,0)
            desctable.push(aux::stringid(ID, 0));
        }
        // if b2 and (opt&2)==0 then
        if b2 && (opt & 2) == 0 {
            idtable.push(2);
            desctable.push(aux::stringid(ID, 1));
        }
        // if b3 and (opt&4)==0 then
        if b3 && (opt & 4) == 0 {
            idtable.push(4);
            desctable.push(aux::stringid(ID, 2));
        }
        // if t==1 then return end
        if idtable.is_empty() {
            return;
        }
        // local op=idtable[Duel.SelectOption(tp,table.unpack(desctable)) + 1]
        let sel = duel::select_option(d, a.tp, true, &desctable);
        let op = idtable[sel as usize];
        // optp=opt+optp
        optp = opt + optp;
        // opt=opt+op
        opt += op;
        // if opt==1 or (optp==2 and opt==3) or (optp==4 and opt==5) or ((optp==8 or optp==10) and opt==7) then
        if opt == 1 || (optp == 2 && opt == 3) || (optp == 4 && opt == 5) || ((optp == 8 || optp == 10) && opt == 7) {
            // s.thop(e,tp,eg,ep,ev,re,r,rp)
            thop(d, a);
        }
        // elseif opt==2 or (optp==1 and opt==3) or (optp==4 and opt==6) or ((optp==6 or optp==9) and opt==7) then
        else if opt == 2 || (optp == 1 && opt == 3) || (optp == 4 && opt == 6) || ((optp == 6 || optp == 9) && opt == 7) {
            // s.desop(e,tp,eg,ep,ev,re,r,rp)
            desop(d, a);
        }
        // elseif opt==4 or (optp==1 and opt==5) or (optp==2 and opt==6) or ((optp==4 or optp==5) and opt==7) then
        else if opt == 4 || (optp == 1 && opt == 5) || (optp == 2 && opt == 6) || ((optp == 4 || optp == 5) && opt == 7) {
            // s.atkop(e,tp,eg,ep,ev,re,r,rp)
            atkop(d, a);
        }
    }
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,3,3,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 3, 3, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local diff=math.abs(Duel.GetLP(tp)-Duel.GetLP(1-tp))
    let diff = (duel::get_lp(d, a.tp) - duel::get_lp(d, 1 - a.tp)).abs() as i64;
    // if c:IsRelateToEffect(e) and c:IsFaceup() and diff>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && diff > 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(math.floor(diff/2))
        effect::set_value(d, e1, diff / 2);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        // e2:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e2, LOCATION_MZONE, 0);
        // e2:SetTarget(s.ftarget)
        effect::set_target(d, e2, Some(effect_card_fn(ftarget)));
        // e2:SetLabel(c:GetFieldID())
        effect::set_label(d, e2, &[card::get_field_id(d, c) as i64]);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3),nil)
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    }
}
/// `s.ftarget(e,c)`: `return e:GetLabel()~=c:GetFieldID()`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
